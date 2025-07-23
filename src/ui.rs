use crate::{App, RefreshState};
use ratatui::{prelude::*, widgets::Paragraph};

pub fn draw(app: &App, frame: &mut Frame) {
    let divs = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1), // Branding and title
            Constraint::Min(1), // Time until next stages
            Constraint::Min(3), // Stage information
            Constraint::Min(1), // Additional Information (Updates, information)
        ])
        .split(frame.area());

    let title = Line::from("IdaCast".bold());

    let schedules = Paragraph::new(format!("{:?}", app.schedules));

    let status = match &app.refresh_state {
        RefreshState::Pending => Line::from("loading schedule data.."),
        RefreshState::Completed(time) => Line::from(format!("schedule data loaded at {time}")),
        RefreshState::Error(report) => Line::from(format!("{report}")),
    };
    frame.render_widget(title, divs[0]);
    frame.render_widget(schedules, divs[2]);
    frame.render_widget(status, divs[3]);
}
