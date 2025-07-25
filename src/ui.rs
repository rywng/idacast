use std::cmp::max;

use crate::{
    app::{App, RefreshState},
    data::{filter_schedules, schedules::Schedule},
};
use chrono::{DateTime, Duration, Local, TimeDelta};
use ratatui::{
    prelude::*,
    widgets::{Block, Paragraph},
};
use unicode_width::UnicodeWidthStr;

pub fn draw(app: &App, frame: &mut Frame) {
    let [header_area, bankara_area, battle_area, footer_area] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Header: Branding and title
            Constraint::Min(5),    // Bankara information
            Constraint::Min(5),    // Other Stage information
            Constraint::Length(1), // Footer: Additional Information (Updates, information)
        ])
        .spacing(1)
        .flex(layout::Flex::SpaceBetween)
        .areas(frame.area());

    // Header
    render_header(app, frame, header_area);
    // Footer
    render_footer(app, frame, footer_area);
    // Stages
    render_stages(app, frame, bankara_area, battle_area);
}

fn render_header(_app: &App, frame: &mut Frame<'_>, header_area: Rect) {
    let time = Local::now().format("%H:%M:%S").to_string().fg(Color::Gray);
    let title = "IdaCast".bold().fg(Color::Green);
    let block = Block::new()
        .title_top(title.into_left_aligned_line())
        .title_top("Splatoon3".fg(Color::Magenta).into_left_aligned_line())
        .title_top(time.into_right_aligned_line());
    frame.render_widget(block, header_area);
}

fn render_footer(app: &App, frame: &mut Frame<'_>, footer_area: Rect) {
    let scroll_info = if app.scroll_offset == 0 || app.schedules_count == 0 {
        "(j/k to scroll)".to_string()
    } else {
        format!(
            "(^L to reset scroll) lines {}/{}",
            app.scroll_offset.saturating_add(1),
            app.schedules_count
                .saturating_sub(app.get_past_schedule_count()),
        )
    }
    .italic()
    .fg(Color::Gray);
    let status = match &app.refresh_state {
        RefreshState::Pending => Span::from("Updating..."),
        RefreshState::Completed(time) => {
            Span::from(format!("Last updated: {}", time.format("%H:%M:%S")))
        }
        RefreshState::Error(report) => Span::from(format!("Failed to update: {report}")),
    }
    .fg(Color::Gray);
    let block = Block::new()
        .title_top(scroll_info.into_right_aligned_line())
        .title_top(status.into_left_aligned_line());
    frame.render_widget(block, footer_area);
}

fn render_stages(app: &App, frame: &mut Frame<'_>, bankara_area: Rect, battle_area: Rect) {
    let [anarchy_series_area, anarchy_open_area] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Fill(1), Constraint::Fill(1)])
        .flex(layout::Flex::SpaceAround)
        .spacing(1)
        .areas(bankara_area);
    let display_count: usize = anarchy_series_area.height as usize / 3;
    // Assuming every block
    // have the same size

    let filtered_open = filter_schedules(
        &app.schedules.anarchy_open,
        display_count,
        Some(app.scroll_offset),
    );
    let filtered_series = filter_schedules(
        &app.schedules.anarchy_series,
        display_count,
        Some(app.scroll_offset),
    );
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

    let [x_battle_area, regular_area] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Fill(1), Constraint::Fill(1)])
        .flex(layout::Flex::SpaceAround)
        .spacing(1)
        .areas(battle_area);
    let x_battle_block = Block::bordered()
        .border_style(Style::new().cyan())
        .title("X Battle");
    let regular_battle_block = Block::bordered()
        .border_style(Style::new().green())
        .title("Regular Battle");

    render_schedule_widget(
        filter_schedules(
            &app.schedules.x_battle,
            display_count,
            Some(app.scroll_offset),
        ),
        x_battle_area,
        x_battle_block,
        frame,
    );
    render_schedule_widget(
        filter_schedules(
            &app.schedules.regular,
            display_count,
            Some(app.scroll_offset),
        ),
        regular_area,
        regular_battle_block,
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
    let l_width = lhs.width_cjk();
    let r_width = rhs.width_cjk();
    let space_count: i32 = area.width as i32 - l_width as i32 - r_width as i32;
    let space_count = max(space_count, 0) as usize;
    " ".repeat(space_count)
}

fn format_stage_times(schedule: &Schedule) -> Span {
    let time_now = Local::now();
    let converted_start_time: DateTime<Local> = DateTime::from(schedule.start_time);
    let converted_end_time: DateTime<Local> = DateTime::from(schedule.end_time);
    let remaining_time = converted_end_time - time_now;
    if remaining_time <= Duration::hours(2) && remaining_time >= TimeDelta::zero() {
        Span::from(
            [
                {
                    if remaining_time.num_hours() != 0 {
                        format!("{}h ", remaining_time.num_hours())
                    } else {
                        "".to_string()
                    }
                },
                format!(
                    "{}m {:.0}s remaining",
                    remaining_time.num_minutes() % 60,
                    (remaining_time.num_milliseconds() % 60000) as f64 / 1000.0,
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
