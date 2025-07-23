use crate::{
    App, RefreshState,
    data::{filter_schedules, schedules::Schedule},
};
use ratatui::{
    prelude::*,
    widgets::{Block, Paragraph},
};

pub fn draw(app: &App, frame: &mut Frame) {
    let [header_area, bankara_area, stages_area, footer_area] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),         // Header: Branding and title
            Constraint::Percentage(50), // Bankara information
            Constraint::Percentage(50), // Other Stage information
            Constraint::Min(1),         // Footer: Additional Information (Updates, information)
        ])
        .areas(frame.area());

    // Header
    let title = Line::from("IdaCast".bold());
    frame.render_widget(title, header_area);

    // Footer
    let status = match &app.refresh_state {
        RefreshState::Pending => Line::from("updating"),
        RefreshState::Completed(time) => Line::from(format!("last update {time}")),
        RefreshState::Error(report) => Line::from(format!("failed to update: {report}")),
    };
    frame.render_widget(status, footer_area);

    // Stages
    const DISPLAY_COUNT: usize = 4;
    let [anarchy_open_area, anarchy_series_area] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .areas(bankara_area);

    let filtered_open = filter_schedules(&app.schedules.anarchy_open, DISPLAY_COUNT);
    let filtered_series = filter_schedules(&app.schedules.anarchy_series, DISPLAY_COUNT);
    let anarchy_open_block = Block::bordered()
        .border_style(Style::new().red())
        .title("Anarchy Open");
    let anarchy_series_block = Block::bordered()
        .border_style(Style::new().red())
        .title("Anarchy Series");

    frame.render_widget(
        get_schedule_widget(filtered_open).block(anarchy_open_block),
        anarchy_open_area,
    );
    frame.render_widget(
        get_schedule_widget(filtered_series).block(anarchy_series_block),
        anarchy_series_area,
    );
}

fn get_schedule_widget(schedules: Option<&[Schedule]>) -> Paragraph {
    match schedules {
        Some(schedules) => {
            let mut text: Vec<Line> = Vec::new();
            for schedule in schedules {
                text.push(Line::from(vec![schedule.rule.name.clone().bold()]));
                for stage in &schedule.stages {
                    text.push(stage.name.clone().into());
                }
            }
            Paragraph::new(text)
        }
        None => Paragraph::new("Loading..."),
    }
}
