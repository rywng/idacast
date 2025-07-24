use std::io::stdout;

use clap::Parser;
use color_eyre::Result;
use crossterm::{event, ExecutableCommand};

mod ui;
mod data;
mod app;

#[derive(Parser, Debug)]
#[command(version, about)]
/// This program displays Splatoon 3's stage data from a terminal user interface.
/// Operating System's language setting is read to automatically determine the translation to use.
struct Args {
    /// Optional language to use when fetching translations. If set, will take precedence over
    /// OS's language setting.
    #[arg(short, long)]
    language: Option<String>,
    /// Mouse capture is enabled by default, so that you can use mouse buttons to manipluate the
    /// display. Supply this option to disable it.
    #[arg(long)]
    no_mouse: Option<bool>,
}

impl Args {
    fn infer_language(&mut self) {
        match &self.language {
            Some(_) => {}
            None => self.language = sys_locale::get_locale(),
        };
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    // Parse arguments and setup
    let mut args = Args::parse();
    args.infer_language();

    let mut terminal = ratatui::init();
    if !args.no_mouse.unwrap_or(false) {
        stdout().execute(event::EnableMouseCapture)?;
    }

    let result = app::App::new(args.language).run(&mut terminal).await;

    ratatui::restore();
    result
}
