use std::cmp::max;

use crate::{
    app::{App, AppScreen, RefreshState},
    data::{
        filter_schedules,
        schedules::{BattleSchedule, CoopSchedule},
    },
};
use chrono::{DateTime, Duration, Local, TimeDelta, Utc};
use ratatui::{
    prelude::*,
    widgets::{Block, Paragraph, Tabs},
};
use strum::IntoEnumIterator;
use unicode_width::UnicodeWidthStr;

use super::AppUI;

pub fn draw(app: &App, frame: &mut Frame) {
    let [header_area, content_area, footer_area] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Header: Branding and title
            Constraint::Min(10),   // Bankara information
            Constraint::Length(1), // Footer: Additional Information (Updates, information)
        ])
        .spacing(1)
        .flex(layout::Flex::SpaceBetween)
        .areas(frame.area());

    render_header(app, frame, header_area);

    render_footer(app, frame, footer_area);

    match app.app_ui.current_screen {
        AppScreen::Battles => render_stages(app, frame, content_area),
        AppScreen::Work => render_work(app, frame, content_area),
        AppScreen::Challenges => {}
        AppScreen::Fest => {}
    }
}

fn render_header(app: &App, frame: &mut Frame<'_>, header_area: Rect) {
    let time = Local::now().format("%H:%M:%S").to_string().fg(Color::Gray);
    let title = "IdaCast".bold().fg(Color::Green);

    let tab_titles = AppScreen::iter().map(AppScreen::to_tab_title);
    let tabs = Tabs::new(tab_titles)
        .highlight_style(Modifier::REVERSED | Modifier::BOLD)
        .select(app.app_ui.current_screen as usize)
        .divider(" ")
        .padding("", "");

    let [title_area, tabs_area, time_area] = Layout::horizontal([
        Constraint::Length(title.content.len() as u16),
        Constraint::Fill(1),
        Constraint::Length(time.content.len() as u16),
    ])
    .direction(Direction::Horizontal)
    .spacing(1)
    .flex(layout::Flex::SpaceAround)
    .areas(header_area);

    frame.render_widget(title, title_area);
    frame.render_widget(tabs, tabs_area);
    frame.render_widget(time, time_area);
}

fn get_scroll_offset(cur_screen: &AppScreen, app_ui: &AppUI) -> usize {
    match cur_screen {
        AppScreen::Battles => app_ui.battles.scroll_offset,
        AppScreen::Work => app_ui.work.scroll_offset,
        AppScreen::Challenges => app_ui.challenges.scroll_offset,
        AppScreen::Fest => app_ui.fest.scroll_offset,
    }
}

fn render_footer(app: &App, frame: &mut Frame<'_>, footer_area: Rect) {
    let scroll_offset = get_scroll_offset(&app.app_ui.current_screen, &app.app_ui);
    let battle_schedules_count = app.app_ui.battles.schedules_count;

    let scroll_info = if scroll_offset == 0 || battle_schedules_count == 0 {
        "(j/k to scroll)".to_string()
    } else {
        format!(
            "(^L to reset scroll) lines {}/{}",
            scroll_offset.saturating_add(1),
            battle_schedules_count.saturating_sub(app.get_past_schedule_count()),
        )
    }
    .italic()
    .fg(Color::Gray);
    let status = match &app.refresh_state {
        RefreshState::Pending => Span::from("Updating..."),
        RefreshState::Completed(time, cached) => {
            Span::from(format!("Last updated: {}{}", time.format("%H:%M:%S"), {
                if *cached { " (cached)" } else { "" }
            }))
        }
        RefreshState::Error(report) => Span::from(format!("Failed to update: {report}")),
    }
    .fg(Color::Gray);

    let [status_area, _spacer, scroll_info_area] = Layout::horizontal([
        Constraint::Length(status.content.len() as u16),
        Constraint::Fill(1),
        Constraint::Length(scroll_info.content.len() as u16),
    ])
    .flex(layout::Flex::SpaceAround)
    .spacing(1)
    .areas(footer_area);

    frame.render_widget(status, status_area);
    frame.render_widget(scroll_info, scroll_info_area);
}

fn render_stages(app: &App, frame: &mut Frame<'_>, stage_area: Rect) {
    let [bankara_area, battle_area] =
        Layout::vertical([Constraint::Min(5), Constraint::Min(5)]).areas(stage_area);

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
        Some(app.app_ui.battles.scroll_offset),
    );
    let filtered_series = filter_schedules(
        &app.schedules.anarchy_series,
        display_count,
        Some(app.app_ui.battles.scroll_offset),
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
            Some(app.app_ui.battles.scroll_offset),
        ),
        x_battle_area,
        x_battle_block,
        frame,
    );
    render_schedule_widget(
        filter_schedules(
            &app.schedules.regular,
            display_count,
            Some(app.app_ui.battles.scroll_offset),
        ),
        regular_area,
        regular_battle_block,
        frame,
    );
}

fn center_single_block(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(layout::Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical])
        .flex(layout::Flex::Center)
        .areas(area);
    area
}

fn render_work(app: &App, frame: &mut Frame, area: Rect) {
    let area = center_single_block(
        area,
        Constraint::Max((area.width as f64 * 0.8).floor() as u16),
        Constraint::Max((area.height as f64 * 0.95).floor() as u16),
    );
    let block = Block::bordered()
        .border_style(Color::Red)
        .title("Grizzco Work");
    render_work_widget(Some(&app.schedules.work_regular[..]), area, block, frame);
}

fn render_work_widget(
    schedules: Option<&[CoopSchedule]>,
    area: Rect,
    block: Block,
    frame: &mut Frame,
) {
    let sub_area = block.inner(area);
    let content = match schedules {
        Some(schedules) => {
            let mut text: Vec<Line> = Vec::new();

            for schedule in schedules {
                let line = format_schedule_title(
                    sub_area,
                    schedule.stage.name.clone(),
                    schedule.start_time,
                    schedule.end_time,
                );
                text.push(line);
                let boss = schedule.boss.name.clone().bold();
                let weapons = schedule
                    .weapons
                    .iter()
                    .map(|weapon| weapon.name.clone())
                    .collect::<Vec<String>>()
                    .join(" / ");
                let mid_space = fill_mid_spaces(&boss.content, &weapons, sub_area);
                text.push(Line::from(vec![
                    weapons.italic(),
                    mid_space.into(),
                    boss.bold(),
                ]));
                text.push(Line::from(""));
            }

            Paragraph::new(text)
        }
        None => Paragraph::new("Loading..."),
    };

    frame.render_widget(content.block(block), area);
}

fn render_schedule_widget(
    schedules: Option<&[BattleSchedule]>,
    area: Rect,
    block: Block,
    frame: &mut Frame,
) {
    let sub_area = block.inner(area);
    let content = match schedules {
        Some(schedules) => {
            let mut text: Vec<Line> = Vec::new();
            for schedule in schedules {
                let line = format_schedule_title(
                    sub_area,
                    schedule.rule.name.clone(),
                    schedule.start_time,
                    schedule.end_time,
                );
                text.push(line);
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

fn format_schedule_title(
    sub_area: Rect,
    name: String,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
) -> Line<'static> {
    let rule = name.clone().bold().underlined();
    let time = format_stage_times(start_time, end_time).italic();
    let space = fill_mid_spaces(&rule.content, &time.content, sub_area).into();
    Line::from(vec![rule, space, time])
}

fn fill_mid_spaces(lhs: &str, rhs: &str, area: Rect) -> String {
    let l_width = lhs.width_cjk();
    let r_width = rhs.width_cjk();
    let space_count: i32 = area.width as i32 - l_width as i32 - r_width as i32;
    let space_count = max(space_count, 0) as usize;
    " ".repeat(space_count)
}

fn format_stage_times(start_time: DateTime<Utc>, end_time: DateTime<Utc>) -> Span<'static> {
    let time_now = Local::now();
    let converted_start_time: DateTime<Local> = DateTime::from(start_time);
    let converted_end_time: DateTime<Local> = DateTime::from(end_time);
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

    use super::fill_mid_spaces;

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
    // TODO: I may need som tests for UI
}
