use std::default;

use color_eyre::{Result, eyre::Ok};
use crossterm::event::{self, Event, EventStream, KeyEvent};
use data::{get_schedules, schedules::Schedules};
use futures::{StreamExt, future::FutureExt};
use ratatui::{DefaultTerminal, prelude::*};

#[derive(Debug, Default)]
struct App {
    exit: bool,
    locale: Option<String>,
    term_event_stream: EventStream,
}

enum AppEvent {
    Tick,
    RequestRefresh,
    Quit,
}

#[derive(Debug, Default)]
enum RefreshState {
    #[default]
    RequestPending,
    Refreshing,
    Completed,
}

#[allow(unused_variables, dead_code)]
mod data;

impl App {
    /// runs the application's main loop until the user quits
    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events().await?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let title = Line::from("Schedule".bold());
        frame.render_widget(title, frame.area());
    }

    async fn handle_events(&mut self) -> Result<()> {
        tokio::select! {
            event = self.term_event_stream.next().fuse() => {
                self.handle_term_event(event.unwrap().unwrap())?;
            }
            _ = tokio::time::sleep(tokio::time::Duration::from_millis(100)) => {
                // Sleep for a short duration to avoid busy waiting.
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
            _ => Ok(()),
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        match key_event.code {
            event::KeyCode::Char(char) => {
                if char == 'q' {
                    self.exit = true;
                }
            }
            event::KeyCode::Esc => {
                self.exit = true;
            }
            _ => {}
        };
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let result = App::default().run(&mut terminal).await;
    ratatui::restore();
    result
}
