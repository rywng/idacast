use core::time;
use std::cmp::max;

use crate::{
    App, RefreshState,
    data::{filter_schedules, schedules::Schedule},
};
use chrono::{DateTime, Duration, Local, TimeDelta};
use ratatui::{
    prelude::*,
    widgets::{Block, Paragraph},
};

pub fn draw(app: &App, frame: &mut Frame) {
    let [header_area, bankara_area, stages_area, footer_area] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),         // Header: Branding and title
            Constraint::Min(16),        // Bankara information
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
    let [anarchy_series_area, anarchy_open_area] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Fill(1), Constraint::Fill(1)])
        .areas(bankara_area);

    let filtered_open = filter_schedules(&app.schedules.anarchy_open, DISPLAY_COUNT);
    let filtered_series = filter_schedules(&app.schedules.anarchy_series, DISPLAY_COUNT);
    let anarchy_open_block = Block::bordered()
        .border_style(Style::new().red())
        .title("Anarchy Open");
    let anarchy_series_block = Block::bordered()
        .border_style(Style::new().red())
        .title("Anarchy Series");

    render_schedule_widget(filtered_open, anarchy_open_area, anarchy_open_block, frame);
    render_schedule_widget(
        filtered_series,
        anarchy_series_area,
        anarchy_series_block,
        frame,
    );
}

fn render_schedule_widget(
    schedules: Option<&[Schedule]>,
    area: Rect,
    block: Block,
    frame: &mut Frame,
) {
    let sub_area = block.inner(area);
    let content = match schedules {
        Some(schedules) => {
            let mut text: Vec<Line> = Vec::new();
            for schedule in schedules {
                let rule = schedule.rule.name.clone().bold().underlined();
                let time = format_stage_times(schedule).italic();
                let space = fill_mid_spaces(&rule.content, &time.content, sub_area).into();
                text.push(Line::from(vec![rule, space, time]));
                for stage in &schedule.stages {
                    text.push(format!("- {}", stage.name).into());
                }
            }
            Paragraph::new(text)
        }
        None => Paragraph::new("Loading..."),
    };
    frame.render_widget(content.block(block), area);
}

fn fill_mid_spaces(lhs: &str, rhs: &str, area: Rect) -> String {
    let len_left: i32 = area.width as i32 - lhs.len() as i32 - rhs.len() as i32;
    let spaces = max(len_left, 0) as usize;
    " ".repeat(spaces)
}

fn format_stage_times(schedule: &Schedule) -> Span {
    let time_now = Local::now();
    let converted_start_time: DateTime<Local> = DateTime::from(schedule.start_time);
    let converted_end_time: DateTime<Local> = DateTime::from(schedule.end_time);
    let remaining_time = converted_end_time - time_now;
    if remaining_time <= Duration::hours(2) && remaining_time >= TimeDelta::zero() {
        Span::from(
            vec![
                {
                    if remaining_time.num_hours() != 0 {
                        format!("{}h ", remaining_time.num_hours())
                    } else {
                        "".to_string()
                    }
                },
                format!(
                    "{}m {}s remaining",
                    remaining_time.num_minutes() % 60,
                    remaining_time.num_seconds() % 60,
                ),
            ]
            .concat(),
        )
    } else {
        format!(
            "{} - {}",
            converted_start_time.format("%H:%M"),
            converted_end_time.format("%H:%M")
        )
        .into()
    }
}

#[cfg(test)]
mod test {
    use core::time;

    use ratatui::{
        layout::Rect,
        style::{Style, Stylize},
        widgets::Block,
    };

    use crate::ui::fill_mid_spaces;

    #[test]
    fn test_fill_mid_spaces() {
        let rule = "Splat Zones".bold().underlined();
        let time = "7m 17s remaining".italic();
        let time_alt = "22:00 - 00:00".italic();
        let area = Rect::new(0, 0, 64, 24);
        let block = Block::bordered()
            .border_style(Style::new().red())
            .title("Anarchy Open");
        let subarea = block.inner(area);

        assert_eq!(
            fill_mid_spaces(&rule.content, &time.content, subarea),
            "                                   ".to_string()
        );
        assert_eq!(
            fill_mid_spaces(&rule.content, &time_alt.content, subarea),
            "                                      ".to_string()
        );
    }
}
