//! # Cricbuzz API Response Format
//!
//! This file just defines the format of the response JSON from Cricbuzz's API.
//! Also contains a few helper methods to access certain fields.

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CricbuzzOverSeparator {
    pub score: u32,
    pub wickets: u32,
    pub innings_id: u32,
    pub o_summary: String,
    pub runs: u32,
    pub bat_striker_ids: Vec<u32>,
    pub bat_striker_names: Vec<String>,
    pub bat_striker_runs: u32,
    pub bat_striker_balls: u32,
    pub bat_non_striker_ids: Vec<u32>,
    pub bat_non_striker_names: Vec<String>,
    pub bat_non_striker_runs: u32,
    pub bat_non_striker_balls: u32,
    pub bowl_ids: Vec<u32>,
    pub bowl_names: Vec<String>,
    pub bowl_overs: f32,
    pub bowl_maidens: u32,
    pub bowl_runs: u32,
    pub bowl_wickets: u32,
    pub timestamp: u64,
    pub over_num: f32,
    pub bat_team_name: String,
    pub event: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CricbuzzBatsmanStriker {
    pub bat_balls: u32,
    pub bat_dots: u32,
    pub bat_fours: u32,
    pub bat_id: u32,
    pub bat_name: String,
    pub bat_mins: u32,
    pub bat_runs: u32,
    pub bat_sixes: u32,
    pub bat_strike_rate: f32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CricbuzzBowlerStriker {
    pub bowl_id: u32,
    pub bowl_name: String,
    pub bowl_maidens: u32,
    pub bowl_noballs: u32,
    pub bowl_ovs: f32,
    pub bowl_runs: u32,
    pub bowl_wides: u32,
    pub bowl_wkts: u32,
    pub bowl_econ: f32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CricbuzzCommentary {
    pub comm_text: String,
    pub timestamp: u64,
    pub ball_nbr: u32,
    pub over_number: f32,
    pub innings_id: u32,
    pub event: String,
    pub bat_team_name: String,
    pub commentary_formats: Vec<String>, // Not sure about data type
    pub over_separator: CricbuzzOverSeparator,
    pub batsman_striker: CricbuzzBatsmanStriker,
    pub bowler_striker: CricbuzzBowlerStriker,
}

#[derive(Debug, Deserialize)]
pub struct CricbuzzMatchHeaderTossResults {
    pub toss_winner_id: u32,
    pub toss_winner_name: String,
    pub decision: String,
}

#[derive(Debug, Deserialize)]
pub struct CricbuzzMatchHeaderResults {
    pub winning_team: String,
    pub win_by_runs: bool,
    pub win_by_innings: bool,
}

#[derive(Debug, Deserialize)]
pub struct CricbuzzMatchHeaderRevisedTarget {
    pub reason: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CricbuzzMatchHeaderMatchTeamInfo {
    pub batting_team_id: u32,
    pub batting_team_short_name: String,
    pub bowling_team_id: u32,
    pub bowling_team_short_name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CricbuzzMatchHeaderTeam {
    pub id: u32,
    pub name: String,
    pub player_details: Vec<String>,
    pub short_name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CricbuzzMatchHeader {
    pub match_id: u32,
    pub match_description: String,
    pub match_format: String,
    pub match_type: String,
    pub complete: bool,
    pub domestic: bool,
    pub match_start_timestamp: u64,
    pub match_complete_timestamp: u64,
    pub day_night: bool,
    pub year: u32,
    pub day_number: u32,
    pub state: String,
    pub status: String,
    pub toss_results: CricbuzzMatchHeaderTossResults,
    pub result: CricbuzzMatchHeaderResults,
    pub revised_target: CricbuzzMatchHeaderRevisedTarget,
    pub players_of_the_match: Vec<String>,
    pub players_of_the_series: Vec<String>,
    pub match_team_info: Vec<CricbuzzMatchHeaderMatchTeamInfo>,
    pub is_match_not_covered: bool,
    pub team1: CricbuzzMatchHeaderTeam,
    pub team2: CricbuzzMatchHeaderTeam,
    pub series_desc: String,
    pub series_id: u32,
    pub series_name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CricbuzzMiniscoreBatsman {
    pub bat_balls: u32,
    pub bat_dots: u32,
    pub bat_fours: u32,
    pub bat_id: u32,
    pub bat_name: String,
    pub bat_mins: u32,
    pub bat_sixes: u32,
    pub bat_strike_rate: f32,
    pub bat_runs: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CricbuzzMiniscoreBatTeam {
    pub team_id: u32,
    pub team_score: u32,
    pub team_wkts: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CricbuzzMiniscoreBowler {
    pub bowl_id: u32,
    pub bowl_name: String,
    pub bowl_maidens: u32,
    pub bowl_noballs: u32,
    pub bowl_ovs: f32,
    pub bowl_runs: u32,
    pub bowl_wides: u32,
    pub bowl_wkts: u32,
    pub bowl_econ: f32,
}

#[derive(Debug, Deserialize)]
pub struct CricbuzzMiniscorePartnership {
    pub balls: u32,
    pub runs: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CricbuzzMiniscoreMatchScoreDetailsInningsScore {
    pub innings_id: u32,
    pub bat_team_id: u32,
    pub bat_team_name: String,
    pub score: u32,
    pub wickets: u32,
    pub overs: f32,
    pub is_declared: bool,
    pub is_follow_on: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CricbuzzMiniscoreMatchScoreDetailsTossResults {
    pub toss_winner_id: u32,
    pub toss_winner_name: String,
    pub decision: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CricbuzzMiniscoreMatchScoreDetailsMatchTeamInfo {
    pub batting_team_id: u32,
    pub batting_team_short_name: String,
    pub bowling_team_id: u32,
    pub bowling_team_short_name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CricbuzzMiniscoreMatchScoreDetails {
    pub match_id: u32,
    pub innings_score_list: Vec<CricbuzzMiniscoreMatchScoreDetailsInningsScore>,
    pub toss_results: CricbuzzMiniscoreMatchScoreDetailsTossResults,
    pub match_team_info: Vec<CricbuzzMiniscoreMatchScoreDetailsMatchTeamInfo>,
    pub is_match_not_covered: bool,
    pub match_format: String,
    pub state: String,
    pub custom_status: String,
    pub highlighted_team_id: u32,
}

#[derive(Debug, Deserialize)]
pub struct CricbuzzMiniscoreLatestPerformance {
    pub runs: u32,
    pub wkts: u32,
    pub label: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CricbuzzMiniscoreMatchUdrs {
    pub match_id: u32,
    pub innings_id: u32,
    pub timestamp: String,
    pub team1_id: u32,
    pub team1_remaining: u32,
    pub team1_successful: u32,
    pub team1_unsuccessful: u32,
    pub team2_id: u32,
    pub team2_remaining: u32,
    pub team2_successful: u32,
    pub team2_unsuccessful: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CricbuzzMiniscore {
    pub innings_id: u32,
    pub batsman_striker: CricbuzzMiniscoreBatsman,
    pub batsman_non_striker: CricbuzzMiniscoreBatsman,
    pub bat_team: CricbuzzMiniscoreBatTeam,
    pub bowler_striker: CricbuzzMiniscoreBowler,
    pub bowler_non_striker: CricbuzzMiniscoreBowler,
    pub overs: f32,
    pub recent_ovs_stats: String,
    pub partner_ship: CricbuzzMiniscorePartnership,
    pub current_run_rate: f32,
    pub required_run_rate: f32,
    pub last_wicket: Option<String>,
    pub match_score_details: CricbuzzMiniscoreMatchScoreDetails,
    pub latest_performance: Vec<CricbuzzMiniscoreLatestPerformance>,
    // ppData: Not parsed
    // TODO: Value is not always present
    // pub match_udrs: CricbuzzMiniscoreMatchUdrs,
    // overSummaryList: Not parsed
    pub overs_rem: Option<f32>,
    pub status: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CricbuzzJson {
    // pub commentary_list: Vec<CricbuzzCommentary>,
    // pub match_header: CricbuzzMatchHeader,
    pub miniscore: CricbuzzMiniscore,
    pub page: String,
    pub enable_no_content: bool,
}

impl CricbuzzJson {
    pub fn bat_striker_name(&self) -> &str {
        self.miniscore.batsman_striker.bat_name.as_str()
    }

    pub fn bat_striker_runs(&self) -> u32 {
        self.miniscore.batsman_striker.bat_runs
    }

    pub fn bat_striker_balls(&self) -> u32 {
        self.miniscore.batsman_striker.bat_balls
    }

    pub fn bat_striker_fours(&self) -> u32 {
        self.miniscore.batsman_striker.bat_fours
    }

    pub fn bat_striker_sixes(&self) -> u32 {
        self.miniscore.batsman_striker.bat_sixes
    }

    pub fn bat_striker_strike_rate(&self) -> f32 {
        self.miniscore.batsman_striker.bat_strike_rate
    }

    pub fn bat_non_striker_name(&self) -> &str {
        self.miniscore.batsman_non_striker.bat_name.as_str()
    }

    pub fn bat_non_striker_runs(&self) -> u32 {
        self.miniscore.batsman_non_striker.bat_runs
    }

    pub fn bat_non_striker_balls(&self) -> u32 {
        self.miniscore.batsman_non_striker.bat_balls
    }

    pub fn bat_non_striker_fours(&self) -> u32 {
        self.miniscore.batsman_non_striker.bat_fours
    }

    pub fn bat_non_striker_sixes(&self) -> u32 {
        self.miniscore.batsman_non_striker.bat_sixes
    }

    pub fn bat_non_striker_strike_rate(&self) -> f32 {
        self.miniscore.batsman_non_striker.bat_strike_rate
    }

    pub fn bowl_striker_name(&self) -> &str {
        self.miniscore.bowler_striker.bowl_name.as_str()
    }

    pub fn bowl_striker_ovs(&self) -> f32 {
        self.miniscore.bowler_striker.bowl_ovs
    }

    pub fn bowl_striker_maidens(&self) -> u32 {
        self.miniscore.bowler_striker.bowl_maidens
    }

    pub fn bowl_striker_runs(&self) -> u32 {
        self.miniscore.bowler_striker.bowl_runs
    }

    pub fn bowl_striker_wkts(&self) -> u32 {
        self.miniscore.bowler_striker.bowl_wkts
    }

    pub fn bowl_striker_econ(&self) -> f32 {
        self.miniscore.bowler_striker.bowl_econ
    }

    pub fn bowl_non_striker_name(&self) -> &str {
        self.miniscore.bowler_non_striker.bowl_name.as_str()
    }

    pub fn bowl_non_striker_ovs(&self) -> f32 {
        self.miniscore.bowler_non_striker.bowl_ovs
    }

    pub fn bowl_non_striker_maidens(&self) -> u32 {
        self.miniscore.bowler_non_striker.bowl_maidens
    }

    pub fn bowl_non_striker_runs(&self) -> u32 {
        self.miniscore.bowler_non_striker.bowl_runs
    }

    pub fn bowl_non_striker_wkts(&self) -> u32 {
        self.miniscore.bowler_non_striker.bowl_wkts
    }

    pub fn bowl_non_striker_econ(&self) -> f32 {
        self.miniscore.bowler_non_striker.bowl_econ
    }

    pub fn partner_ship_runs(&self) -> u32 {
        self.miniscore.partner_ship.runs
    }

    pub fn partner_ship_balls(&self) -> u32 {
        self.miniscore.partner_ship.balls
    }

    pub fn last_wicket(&self) -> &Option<String> {
        &self.miniscore.last_wicket
    }

    pub fn overs_rem(&self) -> Option<f32> {
        self.miniscore.overs_rem
    }

    pub fn toss_winner_name(&self) -> &str {
        &self
            .miniscore
            .match_score_details
            .toss_results
            .toss_winner_name
    }

    pub fn toss_decision(&self) -> &str {
        &self.miniscore.match_score_details.toss_results.decision
    }
}
