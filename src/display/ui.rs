use std::collections::HashMap;

use crate::{app::App, cricbuzz_api::CricbuzzMiniscoreMatchScoreDetailsInningsScore};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Row, Table, Tabs, Wrap},
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
        .constraints(
            [
                Constraint::Length(5),
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ]
            .as_ref(),
        )
        .split(area);

    let curr_match = &app.matches_info[app.focused_tab as usize].cricbuzz_info;

    let scores = get_match_summary_info(curr_match);

    let summ_block = Block::default().borders(Borders::ALL).title("Overview");
    let paragraph = Paragraph::new(scores).block(summ_block);
    f.render_widget(paragraph, chunks[0]);
    draw_live_feed(f, chunks[1], app);
    draw_scorecard(f, chunks[2], app);
}

fn draw_live_feed<B>(f: &mut Frame<B>, area: Rect, app: &App)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
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

    // Drawing Key Stats to the right
    let mut key_stats: Vec<Spans> = vec![];

    key_stats.push(Spans::from(vec![
        Span::styled(
            "Partnership: ",
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::from(format!(
            "{}({})",
            curr_match.miniscore.partner_ship.runs, curr_match.miniscore.partner_ship.balls
        )),
    ]));

    if let Some(l_wkt) = &curr_match.miniscore.last_wicket {
        key_stats.push(Spans::from(vec![
            Span::styled("Last Wkt:", Style::default().add_modifier(Modifier::BOLD)),
            Span::from(l_wkt.as_str()),
        ]));
    }

    if let Some(ovs_rem) = &curr_match.miniscore.overs_rem {
        key_stats.push(Spans::from(vec![
            Span::styled("Ovs Left: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::from(ovs_rem.to_string()),
        ]));
    }

    key_stats.push(Spans::from(vec![
        Span::styled("Toss: ", Style::default().add_modifier(Modifier::BOLD)),
        Span::from(format!(
            "{} ({})",
            curr_match
                .miniscore
                .match_score_details
                .toss_results
                .toss_winner_name,
            curr_match
                .miniscore
                .match_score_details
                .toss_results
                .decision
        )),
    ]));

    let key_stats_block = Block::default().borders(Borders::ALL).title("Key Stats");
    let key_stats_para = Paragraph::new(key_stats)
        .block(key_stats_block)
        .wrap(Wrap { trim: true });
    f.render_widget(key_stats_para, chunks[1]);
}

fn get_match_summary_info(match_info: &CricbuzzJson) -> Vec<Spans> {
    let msd = &match_info.miniscore.match_score_details;
    let mut scores = vec![];

    if msd.match_format == "TEST" {
        let total_inngs = msd.innings_score_list.len();
        if total_inngs == 1 {
            if msd.innings_score_list[0].is_declared {
                scores.push(Spans::from(format!(
                    "{} {}/{} d",
                    msd.innings_score_list[0].bat_team_name.as_str(),
                    msd.innings_score_list[0].score.to_string(),
                    msd.innings_score_list[0].wickets.to_string(),
                )));
            } else {
                scores.push(Spans::from(format!(
                    "{} {}/{}",
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
                    "{} {}/{} ({}) CRR: {}",
                    bat_team_name,
                    teams[bat_team_name][0].score.to_string(),
                    teams[bat_team_name][0].wickets.to_string(),
                    teams[bat_team_name][0].overs.to_string(),
                    match_info.miniscore.current_run_rate.to_string(),
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
                    "{} {}/{} & {}/{} ({}) CRR: {}",
                    bat_team_name,
                    teams[bat_team_name][0].score.to_string(),
                    teams[bat_team_name][0].wickets.to_string(),
                    teams[bat_team_name][1].score.to_string(),
                    teams[bat_team_name][1].wickets.to_string(),
                    teams[bat_team_name][1].overs.to_string(),
                    match_info.miniscore.current_run_rate.to_string(),
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

fn draw_scorecard<B>(f: &mut Frame<B>, area: Rect, app: &App)
where
    B: Backend,
{
    let scorecard = &app.matches_info[app.focused_tab as usize].scorecard;
    let text = vec![Spans::from(format!("{:#?}", scorecard))];

    let block = Block::default().borders(Borders::ALL).title("Scorecard");

    let paragraph = Paragraph::new(text)
        .block(block)
        .wrap(Wrap { trim: true })
        .scroll((
            app.matches_info[app.focused_tab as usize].scorecard_scroll,
            0,
        ));
    f.render_widget(paragraph, area);
}
