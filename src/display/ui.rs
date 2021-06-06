use std::collections::HashMap;

use crate::{
    app::{App, MatchInningsInfo},
    cricbuzz_api::CricbuzzMiniscoreMatchScoreDetailsInningsScore,
};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Row, Table, Tabs, Wrap},
    Frame,
};

/// Stores the UI state i.e. current tab, scroll state of scorecard
pub struct UiState {
    /// Selected tab
    pub focused_tab: usize,
    /// Stores current scroll value and max scroll value for each tab
    pub scrd_scroll: Vec<(u16, u16)>,
}

impl UiState {
    /// Return a new `UiState` struct
    ///
    /// # Arguments
    ///
    /// * `num_tabs` - Number of tabs here is equal to number of live matches
    pub fn new(num_tabs: usize) -> UiState {
        UiState {
            focused_tab: 0,
            scrd_scroll: vec![(0, 0); num_tabs],
        }
    }

    /// Add a value to the `focused_tab` property
    pub fn add_focused_tab(&mut self, value: usize) {
        if self.focused_tab < (self.scrd_scroll.len() - 1) {
            self.focused_tab = self.focused_tab.saturating_add(value);
        }
    }

    /// Subtract a value from the `focused_tab` property
    pub fn sub_focused_tab(&mut self, value: usize) {
        self.focused_tab = self.focused_tab.saturating_sub(value);
    }

    /// Increment the scroll value of a particular tab index
    pub fn add_scrd_scroll(&mut self, value: u16) {
        // Should not cross maximum lines present in the scorecard
        if self.scrd_scroll[self.focused_tab].0 < (self.scrd_scroll[self.focused_tab].1 - 2) {
            self.scrd_scroll[self.focused_tab].0 =
                self.scrd_scroll[self.focused_tab].0.saturating_add(value);
        }
    }

    /// Decrement the scroll value of a particular tab index
    pub fn sub_scrd_scroll(&mut self, value: u16) {
        self.scrd_scroll[self.focused_tab].0 =
            self.scrd_scroll[self.focused_tab].0.saturating_sub(value);
    }

    /// Get the current scroll value
    pub fn current_scroll_value(&self) -> u16 {
        self.scrd_scroll[self.focused_tab].0
    }

    /// Update the max scroll length allowed for a tab
    pub fn update_scroll_max_length(&mut self, value: u16) {
        self.scrd_scroll[self.focused_tab].1 = value;
    }

    /// Update the scorecard scroll vector if any of the matches are not live anymore.
    /// Removes the non-live matches
    pub fn update_on_tick(&mut self, invalid_idx: &[usize]) {
        for i in invalid_idx {
            self.scrd_scroll.remove(*i);
        }
    }
}

/// Renders the UI onto the terminal
pub fn draw_ui<B>(f: &mut Frame<B>, app: &App, ui_state: &mut UiState)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());

    let match_names = app.get_all_matches_short_names();
    let tab_titles = match_names
        .iter()
        .map(|m| Spans::from(Span::styled(m.as_str(), Style::default().fg(Color::White))))
        .collect();

    let tabs = Tabs::new(tab_titles)
        .block(Block::default().borders(Borders::ALL).title("Matches"))
        .highlight_style(Style::default().fg(Color::Green))
        .select(ui_state.focused_tab);
    f.render_widget(tabs, chunks[0]);
    draw_tab(f, chunks[1], app, ui_state);
}

/// Draws the tabs which are the short forms of the live matches
fn draw_tab<B>(f: &mut Frame<B>, area: Rect, app: &App, ui_state: &mut UiState)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Length(5),
                Constraint::Length(9),
                Constraint::Percentage(100),
            ]
            .as_ref(),
        )
        .split(area);

    let scores = get_match_summary_info(app, ui_state);

    let summ_block = Block::default().borders(Borders::ALL).title("Overview");
    let paragraph = Paragraph::new(scores).block(summ_block);
    f.render_widget(paragraph, chunks[0]);
    draw_live_feed(f, chunks[1], app, ui_state);
    draw_scorecard(f, chunks[2], app, ui_state);
}

/// Draws the part showing the currently playing batsmen and bowlers, similar to cricbuzz
fn draw_live_feed<B>(f: &mut Frame<B>, area: Rect, app: &App, ui_state: &mut UiState)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .direction(Direction::Horizontal)
        .split(area);

    let curr_match = app.current_match_cricbuzz_info(ui_state.focused_tab);

    let table = Table::new(vec![
        Row::new(vec!["Batsman", "R", "B", "4", "6", "SR"])
            .style(Style::default().add_modifier(Modifier::BOLD)),
        Row::new(vec![
            curr_match.bat_striker_name().to_string() + " *",
            curr_match.bat_striker_runs().to_string(),
            curr_match.bat_striker_balls().to_string(),
            curr_match.bat_striker_fours().to_string(),
            curr_match.bat_striker_sixes().to_string(),
            curr_match.bat_striker_strike_rate().to_string(),
        ]),
        Row::new(vec![
            curr_match.bat_non_striker_name().to_string(),
            curr_match.bat_non_striker_runs().to_string(),
            curr_match.bat_non_striker_balls().to_string(),
            curr_match.bat_non_striker_fours().to_string(),
            curr_match.bat_non_striker_sixes().to_string(),
            curr_match.bat_non_striker_strike_rate().to_string(),
        ])
        .height(2),
        Row::new(vec!["Bowler", "O", "M", "R", "W", "ECO"])
            .style(Style::default().add_modifier(Modifier::BOLD)),
        Row::new(vec![
            curr_match.bowl_striker_name().to_string() + " *",
            curr_match.bowl_striker_ovs().to_string(),
            curr_match.bowl_striker_maidens().to_string(),
            curr_match.bowl_striker_runs().to_string(),
            curr_match.bowl_striker_wkts().to_string(),
            curr_match.bowl_striker_econ().to_string(),
        ]),
        Row::new(vec![
            curr_match.bowl_non_striker_name().to_string(),
            curr_match.bowl_non_striker_ovs().to_string(),
            curr_match.bowl_non_striker_maidens().to_string(),
            curr_match.bowl_non_striker_runs().to_string(),
            curr_match.bowl_non_striker_wkts().to_string(),
            curr_match.bowl_non_striker_econ().to_string(),
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
            curr_match.partner_ship_runs(),
            curr_match.partner_ship_balls()
        )),
    ]));

    if let Some(l_wkt) = curr_match.last_wicket() {
        key_stats.push(Spans::from(vec![
            Span::styled("Last Wkt: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::from(l_wkt.as_str()),
        ]));
    }

    if let Some(ovs_rem) = &curr_match.overs_rem() {
        key_stats.push(Spans::from(vec![
            Span::styled("Ovs Left: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::from(ovs_rem.to_string()),
        ]));
    }

    key_stats.push(Spans::from(vec![
        Span::styled("Toss: ", Style::default().add_modifier(Modifier::BOLD)),
        Span::from(format!(
            "{} ({})",
            curr_match.toss_winner_name(),
            curr_match.toss_decision()
        )),
    ]));

    let key_stats_block = Block::default().borders(Borders::ALL).title("Key Stats");
    let key_stats_para = Paragraph::new(key_stats)
        .block(key_stats_block)
        .wrap(Wrap { trim: true });
    f.render_widget(key_stats_para, chunks[1]);
}

/// Renders the scores of the teams that are playing
fn get_match_summary_info<'a>(app: &'a App, ui_state: &'a mut UiState) -> Vec<Spans<'a>> {
    let match_info = app.current_match_cricbuzz_info(ui_state.focused_tab);
    let msd = &match_info.miniscore.match_score_details;
    let mut scores = vec![];

    if msd.match_format == "TEST" {
        get_test_match_summary_info(&mut scores, &app, ui_state);
    } else if msd.match_format == "ODI" || msd.match_format == "T20" {
        get_lim_ovs_match_summary_info(&mut scores, &app, ui_state);
    }

    scores.push(Spans::from(Span::styled(
        msd.custom_status.as_str(),
        Style::default()
            .fg(Color::LightRed)
            .add_modifier(Modifier::DIM),
    )));

    scores
}

/// Builds the score summary for a test match
fn get_test_match_summary_info(scores: &mut Vec<Spans>, app: &App, ui_state: &mut UiState) {
    let match_info = app.current_match_cricbuzz_info(ui_state.focused_tab);
    let msd = &match_info.miniscore.match_score_details;

    let total_inngs = msd.innings_score_list.len();
    if total_inngs == 1 {
        if msd.innings_score_list[0].is_declared {
            scores.push(Spans::from(vec![Span::styled(
                format!(
                    "{} {}/{} d ({})",
                    msd.innings_score_list[0].bat_team_name.as_str(),
                    msd.innings_score_list[0].score.to_string(),
                    msd.innings_score_list[0].wickets.to_string(),
                    msd.innings_score_list[0].overs.to_string(),
                ),
                Style::default().add_modifier(Modifier::BOLD),
            )]));
        } else {
            scores.push(Spans::from(vec![Span::styled(
                format!(
                    "{} {}/{} ({}) CRR: {}",
                    msd.innings_score_list[0].bat_team_name.as_str(),
                    msd.innings_score_list[0].score.to_string(),
                    msd.innings_score_list[0].wickets.to_string(),
                    msd.innings_score_list[0].overs.to_string(),
                    match_info.miniscore.current_run_rate.to_string(),
                ),
                Style::default().add_modifier(Modifier::BOLD),
            )]));
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

        let bat_team_name = msd.match_team_info[3].batting_team_short_name.as_str();
        let bowl_team_name = msd.match_team_info[3].bowling_team_short_name.as_str();

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

/// Builds the score summary for an ODI or a T20 match
fn get_lim_ovs_match_summary_info(scores: &mut Vec<Spans>, app: &App, ui_state: &mut UiState) {
    let match_info = app.current_match_cricbuzz_info(ui_state.focused_tab);
    let msd = &match_info.miniscore.match_score_details;

    let total_inngs = msd.innings_score_list.len();
    if total_inngs == 1 {
        scores.push(Spans::from(vec![Span::styled(
            format!(
                "{} {}/{} ({}) CRR: {}",
                msd.innings_score_list[0].bat_team_name,
                msd.innings_score_list[0].score.to_string(),
                msd.innings_score_list[0].wickets.to_string(),
                msd.innings_score_list[0].overs.to_string(),
                match_info.miniscore.current_run_rate.to_string(),
            ),
            Style::default().add_modifier(Modifier::BOLD),
        )]));
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
    }
}

/// Renders the scorecard for a particular match
fn draw_scorecard<B>(f: &mut Frame<B>, area: Rect, app: &App, ui_state: &mut UiState)
where
    B: Backend,
{
    let scorecard = app.current_match_scorecard_info(ui_state.focused_tab);
    let text = format_scorecard_info(scorecard);
    ui_state.update_scroll_max_length(text.len() as u16);

    let block = Block::default().borders(Borders::ALL).title("Scorecard");

    let paragraph = Paragraph::new(text)
        .block(block)
        .wrap(Wrap { trim: true })
        .scroll((ui_state.current_scroll_value(), 0));
    f.render_widget(paragraph, area);
}

/// Returns the structured scorecard information for display on the terminal
///
/// # Arguments
///
/// * `scorecard` - A slice of all the innings information in a match
fn format_scorecard_info(scorecard: &[MatchInningsInfo]) -> Vec<Spans> {
    let mut text = vec![];

    for (ino, info) in scorecard.iter().enumerate().rev() {
        text.push(Spans::from(Span::styled(
            format!("Innings {}", ino + 1),
            Style::default().add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        )));

        text.push(Spans::from(""));

        text.push(Spans::from(Span::styled(
            format!(
                "{:<30} {:<60} {:<3} {:<3} {:<2} {:<2} {:<6}",
                "Batsman", "", "R", "B", "4s", "6s", "SR"
            ),
            Style::default().add_modifier(Modifier::BOLD),
        )));

        for b in &info.batsman_details {
            text.push(Spans::from(format!(
                "{:<30} {:<60} {:<3} {:<3} {:<2} {:<2} {:<6}",
                b.name, b.status, b.runs, b.balls, b.fours, b.sixes, b.strike_rate
            )));
        }

        text.push(Spans::from(""));

        text.push(Spans::from(Span::styled(
            format!(
                "{:<30} {:<5} {:<3} {:<3} {:<2} {:<2} {:<2} {:<6}",
                "Bowler", "O", "M", "R", "W", "NB", "WD", "ECO"
            ),
            Style::default().add_modifier(Modifier::BOLD),
        )));

        for b in &info.bowler_details {
            text.push(Spans::from(format!(
                "{:<30} {:<5} {:<3} {:<3} {:<2} {:<2} {:<2} {:<6}",
                b.name, b.overs, b.maidens, b.runs, b.wickets, b.no_balls, b.wides, b.economy
            )));
        }

        text.push(Spans::from(""));
    }

    text
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::{
        app::{create_match_info, parse_scorecard_from_file, App},
        cricbuzz_api::CricbuzzJson,
        display::ui::{draw_ui, UiState},
    };
    use tui::{backend::TestBackend, buffer::Cell, Terminal};

    // Path is relative to where `cargo test` command is run
    const TEST_FILES_PATH: &str = "./tests/data/";

    // Path is relative to where the testfile is present
    const SNAPSHOTS_PATH: &str = "../../tests/snapshots";

    fn get_terminal(width: u16, height: u16) -> Terminal<TestBackend> {
        let backend = TestBackend::new(width, height);
        let terminal = Terminal::new(backend).unwrap();
        terminal
    }

    fn format_backend(out: Vec<Cell>, width: u16) -> String {
        let mut s = String::new();

        for (i, c) in out.iter().enumerate() {
            if i != 0 && (i % (width as usize)) == 0 {
                s.push('\n');
                s.push_str(c.symbol.as_str());
            } else {
                s.push_str(c.symbol.as_str());
            }
        }
        s
    }

    #[test]
    fn test_odi_first_inngs_draw_ui() {
        let mut app = App::default();

        let scrd_data = fs::read_to_string(format!(
            "{}{}",
            TEST_FILES_PATH, "cricbuzz_odi_scorecard_first_innings.txt"
        ))
        .unwrap();
        let json_data = fs::read_to_string(format!(
            "{}{}",
            TEST_FILES_PATH, "cricbuzz_odi_first_innings.json"
        ))
        .unwrap();

        let json: CricbuzzJson = serde_json::from_str(&json_data).unwrap();
        let mut scorecard = vec![];
        parse_scorecard_from_file(&scrd_data, &mut scorecard);
        let match_short_name = "BAN vs SL".to_string();
        let api_link = "".to_string();

        let match_info = create_match_info(match_short_name, 36096, api_link, json, scorecard);

        app.matches_info.push(match_info);

        let width = 125;
        let height = 35;
        let mut terminal = get_terminal(width, height);
        let mut ui_state = UiState::new(1);

        terminal
            .draw(|mut f| draw_ui(&mut f, &app, &mut ui_state))
            .unwrap();

        let out = terminal.backend().buffer().content().to_vec();
        let out = format_backend(out, width);

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path(SNAPSHOTS_PATH);

        settings.bind(|| {
            insta::assert_display_snapshot!(out);
        });
    }

    #[test]
    fn test_odi_second_inngs_draw_ui() {
        let mut app = App::default();

        let scrd_data = fs::read_to_string(format!(
            "{}{}",
            TEST_FILES_PATH, "cricbuzz_odi_scorecard_second_innings.txt"
        ))
        .unwrap();
        let json_data = fs::read_to_string(format!(
            "{}{}",
            TEST_FILES_PATH, "cricbuzz_odi_second_innings.json"
        ))
        .unwrap();

        let json: CricbuzzJson = serde_json::from_str(&json_data).unwrap();
        let mut scorecard = vec![];
        parse_scorecard_from_file(&scrd_data, &mut scorecard);
        let match_short_name = "BAN vs SL".to_string();
        let api_link = "".to_string();
        let match_id = 36096;

        let match_info = create_match_info(match_short_name, match_id, api_link, json, scorecard);

        app.matches_info.push(match_info);

        let width = 125;
        let height = 35;
        let mut terminal = get_terminal(width, height);
        let mut ui_state = UiState::new(1);

        terminal
            .draw(|mut f| draw_ui(&mut f, &app, &mut ui_state))
            .unwrap();

        let out = terminal.backend().buffer().content().to_vec();
        let out = format_backend(out, width);

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path(SNAPSHOTS_PATH);

        settings.bind(|| {
            insta::assert_display_snapshot!(out);
        });
    }

    #[test]
    fn test_test_first_inngs_draw_ui() {
        let mut app = App::default();

        let scrd_data = fs::read_to_string(format!(
            "{}{}",
            TEST_FILES_PATH, "cricbuzz_test_scorecard_first_innings.txt"
        ))
        .unwrap();
        let json_data = fs::read_to_string(format!(
            "{}{}",
            TEST_FILES_PATH, "cricbuzz_test_first_innings.json"
        ))
        .unwrap();

        let json: CricbuzzJson = serde_json::from_str(&json_data).unwrap();
        let mut scorecard = vec![];
        parse_scorecard_from_file(&scrd_data, &mut scorecard);
        let match_short_name = "ENG vs NZ".to_string();
        let api_link = "".to_string();
        let match_id = 33806;

        let match_info = create_match_info(match_short_name, match_id, api_link, json, scorecard);

        app.matches_info.push(match_info);

        let width = 125;
        let height = 35;
        let mut terminal = get_terminal(width, height);
        let mut ui_state = UiState::new(1);

        terminal
            .draw(|mut f| draw_ui(&mut f, &app, &mut ui_state))
            .unwrap();

        let out = terminal.backend().buffer().content().to_vec();
        let out = format_backend(out, width);

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path(SNAPSHOTS_PATH);

        settings.bind(|| {
            insta::assert_display_snapshot!(out);
        });
    }

    #[test]
    fn test_test_second_inngs_draw_ui() {
        let mut app = App::default();

        let scrd_data = fs::read_to_string(format!(
            "{}{}",
            TEST_FILES_PATH, "cricbuzz_test_scorecard_second_innings.txt"
        ))
        .unwrap();
        let json_data = fs::read_to_string(format!(
            "{}{}",
            TEST_FILES_PATH, "cricbuzz_test_second_innings.json"
        ))
        .unwrap();

        let json: CricbuzzJson = serde_json::from_str(&json_data).unwrap();
        let mut scorecard = vec![];
        parse_scorecard_from_file(&scrd_data, &mut scorecard);
        let match_short_name = "ENG vs NZ".to_string();
        let api_link = "".to_string();
        let match_id = 33806;

        let match_info = create_match_info(match_short_name, match_id, api_link, json, scorecard);

        app.matches_info.push(match_info);

        let width = 125;
        let height = 35;
        let mut terminal = get_terminal(width, height);
        let mut ui_state = UiState::new(1);

        terminal
            .draw(|mut f| draw_ui(&mut f, &app, &mut ui_state))
            .unwrap();

        let out = terminal.backend().buffer().content().to_vec();
        let out = format_backend(out, width);

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path(SNAPSHOTS_PATH);

        settings.bind(|| {
            insta::assert_display_snapshot!(out);
        });
    }

    #[test]
    fn test_test_fourth_inngs_draw_ui() {
        let mut app = App::default();

        let scrd_data = fs::read_to_string(format!(
            "{}{}",
            TEST_FILES_PATH, "cricbuzz_test_scorecard_fourth_innings.txt"
        ))
        .unwrap();
        let json_data = fs::read_to_string(format!(
            "{}{}",
            TEST_FILES_PATH, "cricbuzz_test_fourth_innings.json"
        ))
        .unwrap();

        let json: CricbuzzJson = serde_json::from_str(&json_data).unwrap();
        let mut scorecard = vec![];
        parse_scorecard_from_file(&scrd_data, &mut scorecard);
        let match_short_name = "ENG vs NZ".to_string();
        let api_link = "".to_string();
        let match_id = 33806;

        let match_info = create_match_info(match_short_name, match_id, api_link, json, scorecard);

        app.matches_info.push(match_info);

        let width = 125;
        let height = 35;
        let mut terminal = get_terminal(width, height);
        let mut ui_state = UiState::new(1);

        terminal
            .draw(|mut f| draw_ui(&mut f, &app, &mut ui_state))
            .unwrap();

        let out = terminal.backend().buffer().content().to_vec();
        let out = format_backend(out, width);

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path(SNAPSHOTS_PATH);

        settings.bind(|| {
            insta::assert_display_snapshot!(out);
        });
    }
}
