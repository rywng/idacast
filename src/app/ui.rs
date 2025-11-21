use std::cmp::{max, min};

use crate::{
    app::{App, AppScreen, RefreshState},
    data::{
        filter_schedules,
        schedules::{BattleSchedule, CoopSchedule, LeagueSchedule},
    },
};
use chrono::{DateTime, Duration, Local, SubsecRound, TimeDelta, Utc};
use ratatui::{
    prelude::*,
    widgets::{Block, Paragraph, Tabs, Wrap},
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
        AppScreen::Battles => render_battle_stages(app, frame, content_area),
        AppScreen::Work => render_work(app, frame, content_area),
        AppScreen::Challenges => render_challenges(app, frame, content_area),
        AppScreen::Fest => render_splatfest(app, frame, content_area),
    }
}

fn render_header(app: &App, frame: &mut Frame<'_>, header_area: Rect) {
    let time = Local::now()
        .format("%H:%M:%S <%a>")
        .to_string()
        .fg(Color::Gray);
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

fn render_battle_stages(app: &App, frame: &mut Frame<'_>, stage_area: Rect) {
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
    render_work_widget(
        filter_schedules(
            &app.schedules.work_regular,
            area.height as usize / 3,
            Some(app.app_ui.work.scroll_offset),
        ),
        area,
        block,
        frame,
    );
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

fn render_challenges(app: &App, frame: &mut Frame, area: Rect) {
    if app.schedules.league.is_empty() {
        render_error_widget(
            frame,
            area,
            "No Data.",
            "Either the program is loading, or there won't be a new event challenge anytime soon.",
        );
        return;
    }

    let divided_areas: [_; 2] = if area.width > area.height * 2 {
        // Only need to have horizontal spacing, no need for vertical.
        Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)])
            .flex(layout::Flex::SpaceAround)
            .spacing(1)
            .areas(area)
    } else {
        Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]).areas(area)
    };

    for (index, challenge) in app.schedules.league.iter().enumerate() {
        render_challenge_widget(challenge, divided_areas[index], frame);
    }
}

fn render_challenge_widget(challenge_event: &LeagueSchedule, area: Rect, frame: &mut Frame) {
    let block = Block::bordered()
        .title(Line::from(challenge_event.event_name.name.clone()).centered())
        .title_bottom(Line::from(challenge_event.desc.clone()).right_aligned())
        .border_style(Style::new().magenta());

    let mut content: Vec<Line> = Vec::new();

    content.push(Line::from("~~~~~*****~~~~~").centered());
    let rule_name = &challenge_event.rule.name;
    let maps: Vec<String> = challenge_event
        .stages
        .iter()
        .map(|stage| stage.name.clone())
        .collect();
    let rhs = maps.join(" / ");
    let mid_spaces = "      ";
    content.push(
        Line::from(vec![
            Span::from(rule_name.clone()).underlined().bold(),
            mid_spaces.into(),
            Span::from(rhs).italic(),
        ])
        .centered(),
    );

    content.push("".into());
    for time_period in &challenge_event.time_periods {
        content.push(
            Line::from(format_stage_times(
                time_period.start_time,
                time_period.end_time,
            ))
            .centered(),
        );
    }

    content.push("".into());
    challenge_event
        .details
        .split_terminator("<br />")
        .for_each(|item| content.push(Line::from(item).italic()));

    frame.render_widget(
        Paragraph::new(content)
            .wrap(Wrap { trim: true })
            .block(block),
        area,
    );
}

fn render_splatfest(_app: &App, frame: &mut Frame, area: Rect) {
    render_error_widget(
        frame,
        area,
        "Not Implemented!",
        "This page hasn't been implemented yet. See \n https://github.com/rywng/idacast/issues/9",
    );
}

const ERR_WIDGET_WIDTH: u16 = 48;

fn render_error_widget(frame: &mut Frame<'_>, area: Rect, title: &str, reason: &str) {
    let error_msg = Paragraph::new(vec![
        Line::from(title).bold().centered(),
        Line::from(""),
        Line::from(reason),
    ]);
    frame.render_widget(
        error_msg.wrap(ratatui::widgets::Wrap { trim: true }).block(
            Block::bordered()
                .title("Error")
                .border_style(Style::new().red()),
        ),
        center_single_block(
            area,
            Constraint::Max(ERR_WIDGET_WIDTH),
            Constraint::Max(
                (u16::try_from(reason.len() + 20).unwrap() / min(ERR_WIDGET_WIDTH, area.width - 2))
                    + 4,
            ),
        ),
        // This code doesn't take spaces introduced by wrapping into consideration, but it's good
        // enough.
    );
}

fn format_schedule_title<'a>(
    sub_area: Rect,
    name: String,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
) -> Line<'a> {
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

fn format_stage_times(start_time: DateTime<Utc>, end_time: DateTime<Utc>) -> String {
    let time_now = Local::now().round_subsecs(0); // Due to how this software is run, the time now
    // will be slightly later than a whole second, thus the remaining time will be a bit less than
    // a whole second. Round to the nearest whole second in this case.
    let start_time: DateTime<Local> = DateTime::from(start_time);
    let end_time: DateTime<Local> = DateTime::from(end_time);
    let remaining_time = end_time - time_now;
    if remaining_time <= Duration::hours(2) && remaining_time >= TimeDelta::zero() {
        String::from(
            [
                {
                    if remaining_time.num_hours() != 0 {
                        format!("{}h ", remaining_time.num_hours())
                    } else {
                        "".to_string()
                    }
                },
                format!(
                    "{}m {:>2}s remaining",
                    remaining_time.num_minutes() % 60,
                    remaining_time.num_seconds() % 60,
                ),
            ]
            .concat(),
        )
    } else {
        fn format_time_with_date(time_now: DateTime<Local>, time: DateTime<Local>) -> String {
            if time.date_naive() - time_now.date_naive() >= TimeDelta::weeks(1) {
                time.format("%H:%M <%a %x>").to_string()
            } else {
                time.format("%H:%M <%a>").to_string()
            }
        }

        let start_time_str = if time_now.date_naive() != start_time.date_naive()
            && start_time.date_naive() != end_time.date_naive()
        {
            format_time_with_date(time_now, start_time)
        } else {
            // For example, the battle schedules tomorrow, there's no need to display week day
            // twice, so only display time in start time.
            start_time.format("%H:%M").to_string()
        };
        let end_time_str = if time_now.date_naive() != end_time.date_naive() {
            format_time_with_date(time_now, end_time)
        } else {
            end_time.format("%H:%M").to_string()
        };
        format!("{} - {}", start_time_str, end_time_str)
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
