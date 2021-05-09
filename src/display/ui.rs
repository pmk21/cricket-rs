use std::collections::HashMap;

use crate::{app::App, cricbuzz_api::CricbuzzMiniscoreMatchScoreDetailsInningsScore};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Row, Table, Tabs},
    Frame,
};

use crate::cricbuzz_api::CricbuzzJson;

pub fn draw_ui<B>(f: &mut Frame<B>, app: &App) -> ()
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());

    let tab_titles = app
        .matches_info
        .iter()
        .map(|m| {
            Spans::from(Span::styled(
                m.match_short_name.as_str(),
                Style::default().fg(Color::Green),
            ))
        })
        .collect();

    let tabs = Tabs::new(tab_titles)
        .block(Block::default().borders(Borders::ALL).title("Matches"))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(app.focused_tab as usize);
    f.render_widget(tabs, chunks[0]);
    draw_tab(f, chunks[1], app);
}

fn draw_tab<B>(f: &mut Frame<B>, area: Rect, app: &App)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Length(5), Constraint::Percentage(88)].as_ref())
        .split(area);

    let curr_match = &app.matches_info[app.focused_tab as usize].cricbuzz_info;

    let scores = get_match_summary_info(curr_match);

    let summ_block = Block::default().borders(Borders::ALL).title("Overview");
    let paragraph = Paragraph::new(scores).block(summ_block);
    f.render_widget(paragraph, chunks[0]);
    draw_live_feed(f, chunks[1], app);
}

fn draw_live_feed<B>(f: &mut Frame<B>, area: Rect, app: &App)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .direction(Direction::Horizontal)
        .split(area);

    let curr_match = &app.matches_info[app.focused_tab as usize].cricbuzz_info;

    let table = Table::new(vec![
        Row::new(vec!["Batsman", "R", "B", "4", "6", "SR"])
            .style(Style::default().add_modifier(Modifier::BOLD)),
        Row::new(vec![
            curr_match.miniscore.batsman_striker.bat_name.to_string() + " *",
            curr_match.miniscore.batsman_striker.bat_runs.to_string(),
            curr_match.miniscore.batsman_striker.bat_balls.to_string(),
            curr_match.miniscore.batsman_striker.bat_fours.to_string(),
            curr_match.miniscore.batsman_striker.bat_sixes.to_string(),
            curr_match
                .miniscore
                .batsman_striker
                .bat_strike_rate
                .to_string(),
        ]),
        Row::new(vec![
            curr_match
                .miniscore
                .batsman_non_striker
                .bat_name
                .to_string(),
            curr_match
                .miniscore
                .batsman_non_striker
                .bat_runs
                .to_string(),
            curr_match
                .miniscore
                .batsman_non_striker
                .bat_balls
                .to_string(),
            curr_match
                .miniscore
                .batsman_non_striker
                .bat_fours
                .to_string(),
            curr_match
                .miniscore
                .batsman_non_striker
                .bat_sixes
                .to_string(),
            curr_match
                .miniscore
                .batsman_non_striker
                .bat_strike_rate
                .to_string(),
        ])
        .height(2),
        Row::new(vec!["Bowler", "O", "M", "R", "W", "ECO"])
            .style(Style::default().add_modifier(Modifier::BOLD)),
        Row::new(vec![
            curr_match.miniscore.bowler_striker.bowl_name.to_string() + " *",
            curr_match.miniscore.bowler_striker.bowl_ovs.to_string(),
            curr_match.miniscore.bowler_striker.bowl_maidens.to_string(),
            curr_match.miniscore.bowler_striker.bowl_runs.to_string(),
            curr_match.miniscore.bowler_striker.bowl_wkts.to_string(),
            curr_match.miniscore.bowler_striker.bowl_econ.to_string(),
        ]),
        Row::new(vec![
            curr_match
                .miniscore
                .bowler_non_striker
                .bowl_name
                .to_string(),
            curr_match.miniscore.bowler_non_striker.bowl_ovs.to_string(),
            curr_match
                .miniscore
                .bowler_non_striker
                .bowl_maidens
                .to_string(),
            curr_match
                .miniscore
                .bowler_non_striker
                .bowl_runs
                .to_string(),
            curr_match
                .miniscore
                .bowler_non_striker
                .bowl_wkts
                .to_string(),
            curr_match
                .miniscore
                .bowler_non_striker
                .bowl_econ
                .to_string(),
        ]),
    ])
    .style(Style::default().fg(Color::White))
    .block(Block::default().borders(Borders::ALL).title("Live"))
    .widths(&[
        Constraint::Length(25),
        Constraint::Length(5),
        Constraint::Length(5),
        Constraint::Length(5),
        Constraint::Length(5),
        Constraint::Length(6),
    ]);

    f.render_widget(table, chunks[0]);
}

fn get_match_summary_info(match_info: &CricbuzzJson) -> Vec<Spans> {
    let msd = &match_info.miniscore.match_score_details;
    let mut scores = vec![];

    if msd.match_format == "TEST" {
        let total_inngs = msd.innings_score_list.len();
        if total_inngs == 1 {
            if msd.innings_score_list[0].is_declared == true {
                scores.push(Spans::from(format!(
                    "{} {}/{} d",
                    msd.innings_score_list[0].bat_team_name.as_str(),
                    msd.innings_score_list[0].score.to_string(),
                    msd.innings_score_list[0].wickets.to_string(),
                )));
            } else {
                scores.push(Spans::from(format!(
                    "{} {}/{} d",
                    msd.innings_score_list[0].bat_team_name.as_str(),
                    msd.innings_score_list[0].score.to_string(),
                    msd.innings_score_list[0].wickets.to_string(),
                )));
            }
        } else if total_inngs == 2 {
            let mut teams: HashMap<&str, Vec<&CricbuzzMiniscoreMatchScoreDetailsInningsScore>> =
                HashMap::new();

            let bat_team_name = msd.match_team_info[1].batting_team_short_name.as_str();
            let bowl_team_name = msd.match_team_info[1].bowling_team_short_name.as_str();

            for inns_score in &msd.innings_score_list {
                teams
                    .entry(inns_score.bat_team_name.as_str())
                    .or_insert_with(Vec::new)
                    .push(inns_score);
            }

            scores.push(Spans::from(vec![Span::styled(
                format!(
                    "{} {}/{}",
                    bat_team_name,
                    teams[bat_team_name][0].score.to_string(),
                    teams[bat_team_name][0].wickets.to_string(),
                ),
                Style::default().add_modifier(Modifier::BOLD),
            )]));

            scores.push(Spans::from(vec![Span::styled(
                format!(
                    "{} {}/{}",
                    bowl_team_name,
                    teams[bowl_team_name][0].score.to_string(),
                    teams[bowl_team_name][0].wickets.to_string(),
                ),
                Style::default().fg(Color::DarkGray),
            )]));
        } else if total_inngs == 3 {
            let mut teams: HashMap<&str, Vec<&CricbuzzMiniscoreMatchScoreDetailsInningsScore>> =
                HashMap::new();

            let bat_team_name = msd.match_team_info[2].batting_team_short_name.as_str();
            let bowl_team_name = msd.match_team_info[2].bowling_team_short_name.as_str();

            for inns_score in &msd.innings_score_list {
                teams
                    .entry(inns_score.bat_team_name.as_str())
                    .or_insert_with(Vec::new)
                    .push(inns_score);
            }

            scores.push(Spans::from(vec![Span::styled(
                format!(
                    "{} {}/{} & {}/{}",
                    bat_team_name,
                    teams[bat_team_name][0].score.to_string(),
                    teams[bat_team_name][0].wickets.to_string(),
                    teams[bat_team_name][1].score.to_string(),
                    teams[bat_team_name][1].wickets.to_string(),
                ),
                Style::default().add_modifier(Modifier::BOLD),
            )]));

            scores.push(Spans::from(vec![Span::styled(
                format!(
                    "{} {}/{}",
                    bowl_team_name,
                    teams[bowl_team_name][0].score.to_string(),
                    teams[bowl_team_name][0].wickets.to_string(),
                ),
                Style::default().fg(Color::DarkGray),
            )]));
        } else {
            let mut teams: HashMap<&str, Vec<&CricbuzzMiniscoreMatchScoreDetailsInningsScore>> =
                HashMap::new();

            let bat_team_name = msd.match_team_info[2].batting_team_short_name.as_str();
            let bowl_team_name = msd.match_team_info[2].bowling_team_short_name.as_str();

            for inns_score in &msd.innings_score_list {
                teams
                    .entry(inns_score.bat_team_name.as_str())
                    .or_insert_with(Vec::new)
                    .push(inns_score);
            }

            scores.push(Spans::from(vec![Span::styled(
                format!(
                    "{} {}/{} & {}/{}",
                    bat_team_name,
                    teams[bat_team_name][0].score.to_string(),
                    teams[bat_team_name][0].wickets.to_string(),
                    teams[bat_team_name][1].score.to_string(),
                    teams[bat_team_name][1].wickets.to_string(),
                ),
                Style::default().add_modifier(Modifier::BOLD),
            )]));

            scores.push(Spans::from(vec![Span::styled(
                format!(
                    "{} {}/{} & {}/{}",
                    bowl_team_name,
                    teams[bowl_team_name][0].score.to_string(),
                    teams[bowl_team_name][0].wickets.to_string(),
                    teams[bowl_team_name][1].score.to_string(),
                    teams[bowl_team_name][1].wickets.to_string(),
                ),
                Style::default().fg(Color::DarkGray),
            )]));
        }
    }

    scores.push(Spans::from(Span::styled(
        msd.custom_status.as_str(),
        Style::default()
            .fg(Color::LightRed)
            .add_modifier(Modifier::DIM),
    )));

    scores
}
