use color_eyre::Result;

mod data;

mod ui;

mod app;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let result = app::App::new().run(&mut terminal).await;
    ratatui::restore();
    result
}
