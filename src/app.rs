use std::sync::LazyLock;

use cached::{DiskCache, IOCached};
use chrono::{DateTime, Duration, Local, Utc};
use color_eyre::{Result, eyre::Report};
use crossterm::event::{self, Event, EventStream, KeyEvent, MouseButton, MouseEvent};
use data::schedules::{self};
use futures::{StreamExt, future::FutureExt};
use ratatui::DefaultTerminal;

use tokio::sync::mpsc::UnboundedSender;
use tokio_stream::wrappers::UnboundedReceiverStream;

use data::schedules::Schedules;

use crate::data::{self, get_schedules};
use crate::ui::draw;

// Cache

static CACHE_STORE: LazyLock<DiskCache<String, Schedules>> = LazyLock::new(|| {
    DiskCache::new(CACHE_STORE_NAME)
        .set_lifespan(CACHE_STORE_TTL.to_std().unwrap())
        .set_refresh(false)
        .build()
        .unwrap()
});

// Update the schedules every 4 hours. There's no reason to change it.
const AUTO_UPDATE_INTERVAL: Duration = Duration::hours(2);
const CACHE_STORE_TTL: Duration = Duration::hours(4);
pub const CACHE_STORE_NAME: &str = "IDACAST_CACHE";

pub(crate) struct App {
    pub(crate) exit: bool,
    pub(crate) locale: Option<String>,
    pub(crate) scroll_offset: usize,
    /// the length of the longest schedules fetched, doesn't take account into past schedules
    pub(crate) schedules_count: usize,
    pub(crate) refresh_state: RefreshState,
    pub(crate) schedules: schedules::Schedules,
    pub(crate) appevents_tx: UnboundedSender<AppEvent>,
    pub(crate) appevents_rx: UnboundedReceiverStream<AppEvent>,
    pub(crate) termevents_rx: EventStream,
}

#[derive(Debug)]
pub(crate) enum AppEvent {
    Refresh(RefreshState),
    ScheduleLoad(Schedules),
}

#[derive(Debug, Default)]
pub(crate) enum RefreshState {
    #[default]
    Pending,
    Completed(DateTime<Local>),
    Error(Report),
}

enum ScrollOperation {
    Up,
    Down,
    Reset,
}

fn format_option_string(locale: &Option<String>) -> String {
    match locale {
        Some(locale) => locale.clone(),
        None => "default".to_string(),
    }
}

impl App {
    pub fn new(locale: Option<String>) -> Self {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<AppEvent>();
        App {
            exit: false,
            locale,
            schedules_count: 0,
            scroll_offset: 0,
            refresh_state: RefreshState::Pending,
            termevents_rx: EventStream::new(),
            schedules: Schedules::default(),
            appevents_tx: tx,
            appevents_rx: UnboundedReceiverStream::new(rx),
        }
    }

    fn refresh_schedule(
        tx: UnboundedSender<AppEvent>,
        lang: Option<String>,
        cached: bool,
    ) -> Result<()> {
        tokio::spawn(App::handle_refresh(tx, lang, cached));

        Ok(())
    }

    async fn handle_refresh(
        tx: UnboundedSender<AppEvent>,
        lang: Option<String>,
        cached: bool,
    ) -> Result<()> {
        tx.send(AppEvent::Refresh(RefreshState::Pending))?;

        let cached_opt = App::get_cache(&format_option_string(&lang))?;
        let fetch_online = async || get_schedules(lang).await;

        let schedules_result: Result<Schedules> = if !cached {
            Ok(fetch_online().await?)
        } else if let Some(schedules) = cached_opt {
            Ok(schedules)
        } else {
            Ok(fetch_online().await?)
        };

        match schedules_result {
            Ok(schedules) => {
                tx.send(AppEvent::ScheduleLoad(schedules))?;
                tx.send(AppEvent::Refresh(RefreshState::Completed(Local::now())))?;
            }
            Err(err) => {
                tx.send(AppEvent::Refresh(RefreshState::Error(err)))?;
            }
        }
        Ok(())
    }

    fn get_cache(lang: &str) -> Result<Option<Schedules>> {
        Ok(CACHE_STORE.cache_get(&lang.to_string())?)
    }

    /// runs the application's main loop until the user quits
    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        App::register_auto_update(self.appevents_tx.clone(), self.locale.clone())?;
        while !self.exit {
            terminal.draw(|frame| draw(self, frame))?;
            self.handle_events().await?;
        }
        Ok(())
    }

    fn register_auto_update(tx: UnboundedSender<AppEvent>, locale: Option<String>) -> Result<()> {
        tokio::spawn(App::handle_auto_update(tx.clone(), locale.clone()));

        Ok(())
    }

    async fn handle_auto_update(
        tx: UnboundedSender<AppEvent>,
        locale: Option<String>,
    ) -> Result<()> {
        let mut interval = tokio::time::interval(AUTO_UPDATE_INTERVAL.to_std()?);

        loop {
            interval.tick().await;
            App::refresh_schedule(tx.clone(), locale.clone(), true)?;
        }
    }

    async fn handle_events(&mut self) -> Result<()> {
        // Re-draw the TUI every second to update the clock
        let sleep_duration_until_next_second = {
            let time_now = Utc::now();
            let sleep_duration: std::time::Duration =
                (DateTime::from_timestamp(time_now.timestamp() + 1, 0).unwrap() - time_now)
                    .to_std()?;
            sleep_duration
        };

        tokio::select! {
            term_event = self.termevents_rx.next().fuse() => {
                self.handle_term_event(term_event.unwrap().unwrap())?;
            }
            app_event = self.appevents_rx.next().fuse() => {
                self.handle_app_event(app_event.unwrap())?;
            }
            _ = tokio::time::sleep(sleep_duration_until_next_second) => {
                // Sleep each second to keep the clock going
            }
        }
        Ok(())
    }

    fn handle_app_event(&mut self, event: AppEvent) -> Result<()> {
        match event {
            AppEvent::Refresh(refresh_state) => self.refresh_state = refresh_state,
            AppEvent::ScheduleLoad(schedules) => {
                if self.schedules != schedules {
                    self.schedules = schedules.clone();
                    self.schedules_count = self.get_schedules_count().unwrap_or(0);
                    CACHE_STORE.cache_set(format_option_string(&self.locale), schedules)?;
                }
            }
        }

        Ok(())
    }

    fn handle_term_event(&mut self, event: Event) -> Result<()> {
        match event {
            Event::Key(key_event) => {
                self.handle_key_event(key_event)?;
                Ok(())
            }
            Event::Mouse(mouse_event) => {
                self.handle_mouse_event(mouse_event)?;
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn handle_mouse_event(&mut self, mouse_event: MouseEvent) -> Result<()> {
        match mouse_event.kind {
            crossterm::event::MouseEventKind::Down(MouseButton::Right) => {
                self.handle_scroll(ScrollOperation::Reset);
                Ok(())
            }
            crossterm::event::MouseEventKind::ScrollDown => {
                self.handle_scroll(ScrollOperation::Down);
                Ok(())
            }
            crossterm::event::MouseEventKind::ScrollUp => {
                self.handle_scroll(ScrollOperation::Up);
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        match key_event.modifiers {
            event::KeyModifiers::CONTROL => {
                if let event::KeyCode::Char(char) = key_event.code {
                    match char {
                        'l' => self.handle_scroll(ScrollOperation::Reset),
                        'c' => self.quit(),
                        _ => {}
                    }
                }
            }
            event::KeyModifiers::NONE => match key_event.code {
                event::KeyCode::Char(char) => match char {
                    'q' => self.quit(),
                    'r' => App::refresh_schedule(
                        self.appevents_tx.clone(),
                        self.locale.clone(),
                        false,
                    )?,
                    'k' => self.handle_scroll(ScrollOperation::Up),
                    'j' => self.handle_scroll(ScrollOperation::Down),
                    _ => {}
                },
                event::KeyCode::Esc => {
                    self.quit();
                }
                _ => {}
            },
            _ => {}
        }
        Ok(())
    }

    fn handle_scroll(&mut self, operation: ScrollOperation) {
        match operation {
            ScrollOperation::Up => {
                self.scroll_offset = self
                    .scroll_offset
                    .saturating_sub(1)
                    .clamp(0, self.get_clamp_upper());
            }
            ScrollOperation::Down => {
                self.scroll_offset = self
                    .scroll_offset
                    .saturating_add(1)
                    .clamp(0, self.get_clamp_upper());
            }
            ScrollOperation::Reset => {
                self.scroll_offset = 0;
            }
        }
    }

    fn quit(&mut self) {
        self.exit = true;
    }

    fn get_clamp_upper(&self) -> usize {
        self.schedules_count
            .saturating_sub(1 + (self.get_past_schedule_count()))
    }

    fn get_schedules_count(&self) -> Option<usize> {
        let counts = [
            self.schedules.regular.len(),
            self.schedules.anarchy_open.len(),
            self.schedules.anarchy_series.len(),
            self.schedules.x_battle.len(),
        ];

        counts.iter().max().copied()
    }

    pub(crate) fn get_past_schedule_count(&self) -> usize {
        // the logic is too convoluted, may need a rewrite
        let earliest_schedule = self.schedules.regular.first().or_else(|| {
            self.schedules.anarchy_open.first().or_else(|| {
                self.schedules
                    .anarchy_series
                    .first()
                    .or_else(|| self.schedules.x_battle.first())
            })
        });
        match earliest_schedule {
            Some(schedule) => {
                let time_delta = Utc::now() - schedule.start_time;
                time_delta.num_hours().unsigned_abs() as usize / 2
            }
            None => 0,
        }
    }
}
