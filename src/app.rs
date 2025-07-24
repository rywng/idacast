use chrono::{DateTime, Local, Utc};
use color_eyre::{Result, eyre::Report};
use crossterm::event::KeyEvent;
use crossterm::event::{self};
use data::schedules::{self};
use futures::{StreamExt, future::FutureExt};
use ratatui::DefaultTerminal;

use tokio::sync::mpsc::UnboundedSender;
use tokio_stream::wrappers::UnboundedReceiverStream;

use crossterm::event::Event;

use crossterm::event::EventStream;

use data::schedules::Schedules;

use crate::data::{self, get_schedules};
use crate::ui::draw;

// Update the schedules every 4 hours. There's no reason to change it.
const AUTO_UPDATE_HOURS: u8 = 4;

#[derive(Debug)]
pub(crate) struct App {
    pub(crate) exit: bool,
    pub(crate) locale: Option<String>,
    pub(crate) refresh_state: RefreshState,
    pub(crate) schedules: schedules::Schedules,
    pub(crate) appevents_tx: UnboundedSender<AppEvent>,
    pub(crate) appevents_rx: UnboundedReceiverStream<AppEvent>,
    pub(crate) termevents_rx: EventStream,
}

#[derive(Debug)]
pub(crate) enum AppEvent {
    Tick,
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

impl App {
    pub fn new(locale: Option<String>) -> Self {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<AppEvent>();
        App {
            exit: false,
            locale,
            refresh_state: RefreshState::Pending,
            termevents_rx: EventStream::new(),
            schedules: Schedules::default(),
            appevents_tx: tx,
            appevents_rx: UnboundedReceiverStream::new(rx),
        }
    }

    pub fn refresh_schedule(tx: UnboundedSender<AppEvent>, lang: Option<String>) -> Result<()> {
        // TODO: Error handling
        tokio::spawn(App::handle_refresh(tx, lang));

        Ok(())
    }

    pub(crate) async fn handle_refresh(
        tx: UnboundedSender<AppEvent>,
        lang: Option<String>,
    ) -> Result<()> {
        tx.send(AppEvent::Refresh(RefreshState::Pending))?;

        match get_schedules(lang).await {
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
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(
            60 * 60 * AUTO_UPDATE_HOURS as u64,
        ));

        loop {
            interval.tick().await;
            App::refresh_schedule(tx.clone(), locale.clone())?;
        }
    }

    pub(crate) async fn handle_events(&mut self) -> Result<()> {
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
                // Sleep each sercond to keep the clock going
            }
        }
        Ok(())
    }

    pub(crate) fn handle_app_event(&mut self, event: AppEvent) -> Result<()> {
        match event {
            AppEvent::Tick => todo!(),
            AppEvent::Refresh(refresh_state) => self.refresh_state = refresh_state,
            AppEvent::ScheduleLoad(schedules) => self.schedules = schedules,
        }

        Ok(())
    }

    pub(crate) fn handle_term_event(&mut self, event: crossterm::event::Event) -> Result<()> {
        match event {
            Event::Key(key_event) => {
                self.handle_key_event(key_event)?;
                Ok(())
            }
            _ => Ok(()),
        }
    }

    pub(crate) fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        match key_event.code {
            event::KeyCode::Char(char) => match char {
                'q' => self.quit(),
                'r' => App::refresh_schedule(self.appevents_tx.clone(), self.locale.clone())?,
                _ => {}
            },
            event::KeyCode::Esc => {
                self.quit();
            }
            _ => {}
        };
        Ok(())
    }

    pub(crate) fn quit(&mut self) {
        self.exit = true;
    }
}
