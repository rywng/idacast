use clap::Parser;
use color_eyre::Result;

mod data;

mod ui;

mod app;

#[derive(Parser, Debug)]
#[command(version, about)]
/// This program displays Splatoon 3's stage data from a terminal user interface.
/// Operating System's language setting is read to automatically determine the translation to use.
struct Args {
    /// Optional language to use when fetching translations. If set, will take precidence over
    /// OS's language setting.
    #[arg(short, long)]
    language: Option<String>,
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
    let mut args = Args::parse();
    args.infer_language();

    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let result = app::App::new(args.language).run(&mut terminal).await;
    ratatui::restore();
    result
}
