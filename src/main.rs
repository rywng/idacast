use color_eyre::{Result, eyre::Ok};
use ratatui::{
    crossterm::event::{self, Event, KeyEvent}, prelude::*, DefaultTerminal
};

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
}

#[allow(unused_variables, dead_code)]
mod data;

impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let title = Line::from("Schedule".bold());
        frame.render_widget(title, frame.area());
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
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

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let result = App::default().run(&mut terminal);
    ratatui::restore();
    result
}
