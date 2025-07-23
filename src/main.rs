use chrono::{DateTime, Local, Utc};
use color_eyre::{Result, eyre::Report};
use crossterm::event::{self, Event, EventStream, KeyEvent};
use data::{
    get_schedules,
    schedules::{self, Schedules},
};
use futures::{StreamExt, future::FutureExt};
use ratatui::DefaultTerminal;

mod data;

mod ui;
use tokio::sync::mpsc::UnboundedSender;
use tokio_stream::wrappers::UnboundedReceiverStream;
use ui::draw;

#[derive(Debug)]
struct App {
    exit: bool,
    locale: Option<String>,
    refresh_state: RefreshState,
    schedules: schedules::Schedules,
    appevents_tx: UnboundedSender<AppEvent>,
    appevents_rx: UnboundedReceiverStream<AppEvent>,
    termevents_rx: EventStream,
}

#[derive(Debug)]
enum AppEvent {
    Tick,
    Refresh(RefreshState),
    ScheduleLoad(Schedules),
}

#[derive(Debug, Default)]
enum RefreshState {
    #[default]
    Pending,
    Completed(DateTime<Local>),
    Error(Report),
}

impl App {
    pub fn new() -> Self {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<AppEvent>();
        App {
            exit: false,
            locale: None,
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

    async fn handle_refresh(
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
        App::refresh_schedule(self.appevents_tx.clone(), self.locale.clone())?;
        while !self.exit {
            terminal.draw(|frame| draw(self, frame))?;
            self.handle_events().await?;
        }
        Ok(())
    }

    async fn handle_events(&mut self) -> Result<()> {
        // Update every second
        let time_now = Utc::now();
        let sleep_duration: std::time::Duration =
            (DateTime::from_timestamp(time_now.timestamp() + 1, 0).unwrap() - time_now).to_std()?;

        tokio::select! {
            term_event = self.termevents_rx.next().fuse() => {
                self.handle_term_event(term_event.unwrap().unwrap())?;
            }
            app_event = self.appevents_rx.next().fuse() => {
                self.handle_app_event(app_event.unwrap())?;
            }
            _ = tokio::time::sleep(sleep_duration) => {
                // Sleep each sercond to keep the clock going
            }
        }
        Ok(())
    }

    fn handle_app_event(&mut self, event: AppEvent) -> Result<()> {
        match event {
            AppEvent::Tick => todo!(),
            AppEvent::Refresh(refresh_state) => self.refresh_state = refresh_state,
            AppEvent::ScheduleLoad(schedules) => self.schedules = schedules,
        }

        Ok(())
    }

    fn handle_term_event(&mut self, event: crossterm::event::Event) -> Result<()> {
        match event {
            Event::Key(key_event) => {
                self.handle_key_event(key_event)?;
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
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

    fn quit(&mut self) {
        self.exit = true;
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let result = App::new().run(&mut terminal).await;
    ratatui::restore();
    result
}
