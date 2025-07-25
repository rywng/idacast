use std::io::stdout;

use app::CACHE_STORE_NAME;
use cached::DiskCache;
use clap::Parser;
use color_eyre::Result;
use crossterm::{ExecutableCommand, event};
use data::schedules::Schedules;

mod app;
mod data;
mod ui;

#[derive(Parser, Debug)]
#[command(version, about)]
/// This program displays Splatoon 3's stage data from a terminal user interface.
/// Operating System's language setting is read to automatically determine the translation to use.
/// Data is fetched from https://splatoon3.ink/
struct Args {
    /// Optional language to use when fetching translations. If set, will take precedence over
    /// OS's language setting.
    #[arg(short, long)]
    language: Option<String>,
    /// Mouse capture is enabled by default, so that you can use mouse buttons to manipluate the
    /// display. Supply this option to disable it.
    #[arg(long)]
    no_mouse: bool,
    /// Tries to clear the network cache
    #[arg(long)]
    clear_cache: bool,
}

impl Args {
    fn infer_language(&mut self) {
        match &self.language {
            Some(_) => {}
            None => self.language = sys_locale::get_locale(),
        };
    }
}

fn clear_cache() -> Result<()> {
    let mut cache_db = DiskCache::<String, Schedules>::new(CACHE_STORE_NAME).build()?;
    let connection = cache_db.connection_mut();
    connection.clear()?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    // Parse arguments and setup
    let mut args = Args::parse();
    args.infer_language();

    if args.clear_cache {
        return clear_cache();
    }

    let mut terminal = ratatui::init();
    if !args.no_mouse {
        stdout().execute(event::EnableMouseCapture)?;
    }

    let result = app::App::new(args.language).run(&mut terminal).await;

    ratatui::restore();
    stdout().execute(event::DisableMouseCapture)?;
    result
}
