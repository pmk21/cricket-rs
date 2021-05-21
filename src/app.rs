use select::predicate::{Class, Name, Predicate};
use select::{document::Document, node::Node, predicate::Attr};

use crate::cricbuzz_api::CricbuzzJson;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct BowlerInfo {
    pub name: String,
    pub overs: String,
    pub maidens: String,
    pub runs: String,
    pub wickets: String,
    pub no_balls: String,
    pub wides: String,
    pub economy: String,
}

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct BatsmanInfo {
    pub name: String,
    pub status: String,
    pub runs: String,
    pub balls: String,
    pub fours: String,
    pub sixes: String,
    pub strike_rate: String,
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct MatchInningsInfo {
    pub batsman_details: Vec<BatsmanInfo>,
    pub yet_to_bat: String,
    pub bowler_details: Vec<BowlerInfo>,
}

pub struct MatchInfo {
    pub match_short_name: String,
    pub cricbuzz_match_id: u32,
    pub cricbuzz_match_api_link: String,
    pub cricbuzz_info: CricbuzzJson,
    pub scorecard: Vec<MatchInningsInfo>,
    pub scorecard_scroll: u16,
}

pub struct App {
    pub matches_info: Vec<MatchInfo>,
    pub focused_tab: usize,
}

const CRICBUZZ_URL: &str = "https://www.cricbuzz.com";
const CRICBUZZ_MATCH_API: &str = "https://www.cricbuzz.com/api/cricket-match/commentary/";
const CRICBUZZ_MATCH_SCORECARD_API: &str = "https://www.cricbuzz.com/api/html/cricket-scorecard/";

impl App {
    pub async fn new() -> App {
        let mut matches_info = vec![];
        let mut match_name_id: Vec<(String, String)> = vec![];
        let mut scorecard: Vec<MatchInningsInfo> = vec![];

        match get_all_live_matches_id_and_short_name().await {
            Ok(v) => match_name_id = v,
            Err(e) => {
                println!("{:?}", e);
            }
        };

        for (name, id) in &match_name_id {
            let match_id: u32 = id.parse().unwrap();
            if let Ok(json) = get_match_info_from_id(match_id).await {
                if prepare_scorecard(match_id, &mut scorecard).await.is_ok() {
                    matches_info.push(MatchInfo::new(
                        name.to_string(),
                        match_id,
                        format!("{}{}", String::from(CRICBUZZ_MATCH_API), match_id),
                        json,
                        scorecard.clone(),
                    ));
                    scorecard.clear();
                } else {
                    matches_info.push(MatchInfo::new(
                        name.to_string(),
                        match_id,
                        format!("{}{}", String::from(CRICBUZZ_MATCH_API), match_id),
                        json,
                        vec![],
                    ));
                }
            }
        }

        let focused_tab = 0;

        App {
            matches_info,
            focused_tab,
        }
    }

    pub async fn update_on_tick(&mut self) {
        // WARN: Update algorithm might be slow for larger number of matches
        // For now this should be fine and not cause any bottlenecks
        let mut match_name_id: Vec<(String, String)> = vec![];
        let mut scorecard: Vec<MatchInningsInfo> = vec![];

        match get_all_live_matches_id_and_short_name().await {
            Ok(v) => match_name_id = v,
            Err(e) => {
                println!("{:?}", e);
            }
        };

        let mut non_live_matches_idx: Vec<usize> = vec![];
        for (idx, mi) in &mut self.matches_info.iter_mut().enumerate() {
            if match_name_id
                .iter()
                .any(|e| e.1 == mi.cricbuzz_match_id.to_string())
            {
                let mid = mi.cricbuzz_match_id;
                if let Ok(json) = get_match_info_from_id(mid).await {
                    if prepare_scorecard(mid, &mut scorecard).await.is_ok() {
                        mi.cricbuzz_info = json;
                        mi.scorecard = scorecard.clone();
                        scorecard.clear();
                    }
                }
            } else {
                non_live_matches_idx.push(idx);
            }
        }

        for i in non_live_matches_idx {
            self.matches_info.remove(i);
        }
    }

    pub fn get_all_matches_short_names(&self) -> Vec<String> {
        let names: Vec<String> = self
            .matches_info
            .iter()
            .map(|m| m.match_short_name.clone())
            .collect();
        names
    }

    pub fn current_match_cricbuzz_info(&self) -> &CricbuzzJson {
        &self.matches_info[self.focused_tab].cricbuzz_info
    }

    pub fn current_match_scorecard_info(&self) -> &Vec<MatchInningsInfo> {
        &self.matches_info[self.focused_tab].scorecard
    }
}

impl MatchInfo {
    fn new(
        match_short_name: String,
        cricbuzz_match_id: u32,
        cricbuzz_match_api_link: String,
        cricbuzz_info: CricbuzzJson,
        scorecard: Vec<MatchInningsInfo>,
    ) -> MatchInfo {
        MatchInfo {
            match_short_name,
            cricbuzz_match_id,
            cricbuzz_match_api_link,
            cricbuzz_info,
            scorecard,
            scorecard_scroll: 0,
        }
    }
}

// TODO: Need to improve method of getting all matches
async fn get_all_live_matches_id_and_short_name(
) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    let resp_html = reqwest::get(CRICBUZZ_URL).await?.text().await?;
    let mut match_id_name = vec![];

    parse_all_live_matches_id_and_short_name(&resp_html, &mut match_id_name);

    Ok(match_id_name)
}

fn parse_all_live_matches_id_and_short_name(html: &str, match_id_name: &mut Vec<(String, String)>) {
    let document = Document::from(html);

    for node in document.find(Class("cb-mat-mnu").descendant(Name("a"))) {
        // This check might break in the future
        if !node.text().is_empty() && !node.text().eq("MATCHES") {
            if let Some(text) = node.text().split('-').nth(1) {
                if text.trim().eq("Live") {
                    if let Some(link) = node.attr("href") {
                        let split_str: Vec<&str> = link.split('/').collect();
                        match_id_name.push((node.text(), String::from(split_str[2])));
                    }
                }
            }
        }
    }
}

async fn get_match_info_from_id(match_id: u32) -> Result<CricbuzzJson, Box<dyn std::error::Error>> {
    let resp = reqwest::get(format!("{}{}", String::from(CRICBUZZ_MATCH_API), match_id))
        .await?
        .text()
        .await?;

    let res: CricbuzzJson = serde_json::from_str(&resp).unwrap();
    Ok(res)
}

async fn prepare_scorecard(
    id: u32,
    scorecard: &mut Vec<MatchInningsInfo>,
) -> Result<(), Box<dyn std::error::Error>> {
    let resp_html = reqwest::get(format!(
        "{}{}",
        String::from(CRICBUZZ_MATCH_SCORECARD_API),
        id
    ))
    .await?
    .text()
    .await?;

    parse_scorecard(&resp_html, scorecard);

    Ok(())
}

fn parse_scorecard(html: &str, scorecard: &mut Vec<MatchInningsInfo>) {
    let document = Document::from(html);

    for i in 1..5 {
        if let Some(inngs) = document
            .find(Attr("id", format!("innings_{}", i.to_string()).as_ref()))
            .next()
        {
            populate_innings_info(&inngs, scorecard);
        }
    }
}

fn populate_innings_info(inngs: &Node, scorecard: &mut Vec<MatchInningsInfo>) {
    // WARNING: This scorecard parsing function might break easily!

    let mut match_inngs_info = MatchInningsInfo::default();

    let mut count = 0;

    // TODO: Split this into functions
    // Prints each batsman's scorecard info
    let bat_info = inngs.children().nth(1).unwrap();
    let mut batsman_info = BatsmanInfo::default();
    for node in bat_info.children().skip(4) {
        if node.attr("class").is_some() && node.children().count() == 15 {
            for inner_node in node.children() {
                if !inner_node.text().trim().is_empty() {
                    if count == 0 {
                        batsman_info.name = inner_node.text().trim().to_string();
                    } else if count == 1 {
                        batsman_info.status = inner_node.text().trim().to_string();
                    } else if count == 2 {
                        batsman_info.runs = inner_node.text().trim().to_string();
                    } else if count == 3 {
                        batsman_info.balls = inner_node.text().trim().to_string();
                    } else if count == 4 {
                        batsman_info.fours = inner_node.text().trim().to_string();
                    } else if count == 5 {
                        batsman_info.sixes = inner_node.text().trim().to_string();
                    } else if count == 6 {
                        batsman_info.strike_rate = inner_node.text().trim().to_string();
                    }

                    count += 1;
                }
            }
            count = 0;
            match_inngs_info.batsman_details.push(batsman_info.clone());
        }
    }

    // Fall of Wickets
    // let fow_hdr = inngs.children().nth(3).unwrap();
    // println!("{}", fow_hdr.text());

    // Shows all wickets fallen
    // let fow_all = inngs.children().nth(4).unwrap();
    // println!("{}", fow_all.text());

    // TODO: Split this into functions
    // Bowler Scorecard Info - Bowler O M R W NB WD ECO
    count = 0;
    if let Some(bowl_info) = inngs.children().nth(6) {
        let mut bowler_info = BowlerInfo::default();
        for n in bowl_info.children().skip(2) {
            if n.attr("class").is_some() {
                for n1 in n.children() {
                    if !n1.text().trim().is_empty() {
                        if count == 0 {
                            bowler_info.name = n1.text().trim().to_string();
                        } else if count == 1 {
                            bowler_info.overs = n1.text().trim().to_string();
                        } else if count == 2 {
                            bowler_info.maidens = n1.text().trim().to_string();
                        } else if count == 3 {
                            bowler_info.runs = n1.text().trim().to_string();
                        } else if count == 4 {
                            bowler_info.wickets = n1.text().trim().to_string();
                        } else if count == 5 {
                            bowler_info.no_balls = n1.text().trim().to_string();
                        } else if count == 6 {
                            bowler_info.wides = n1.text().trim().to_string();
                        } else if count == 7 {
                            bowler_info.economy = n1.text().trim().to_string();
                        }

                        count += 1;
                    }
                }
                match_inngs_info.bowler_details.push(bowler_info.clone());
                count = 0;
            }
        }
    }

    scorecard.push(match_inngs_info);
}

#[cfg(test)]
mod tests {
    use std::fs;

    use insta;

    use crate::app::{parse_all_live_matches_id_and_short_name, parse_scorecard};

    // Path is relative to where `cargo test` command is run
    const TEST_FILES_PATH: &str = "./tests/data/";

    // Path is relative to where the testfile is present
    const SNAPSHOTS_PATH: &str = "../tests/snapshots";

    #[test]
    fn test_parse_all_live_matches_id_and_short_name_four_live_matches() {
        let fp = format!("{}{}", TEST_FILES_PATH, "cricbuzz_home_four_live.txt");
        let html = fs::read_to_string(fp).unwrap();

        let res_match_id_name: Vec<(String, String)> = vec![
            ("KENT vs GLAM - Live".to_string(), "33238".to_string()),
            ("HAM vs LEIC - Live".to_string(), "33243".to_string()),
            ("SUR vs MDX - Live".to_string(), "33253".to_string()),
            ("GLOUCS vs SOM - Live".to_string(), "33248".to_string()),
        ];
        let mut match_id_name = vec![];

        parse_all_live_matches_id_and_short_name(&html, &mut match_id_name);

        assert_eq!(res_match_id_name, match_id_name);
    }

    #[test]
    fn test_parse_all_live_matches_id_and_short_name_no_live_matches() {
        let fp = format!("{}{}", TEST_FILES_PATH, "cricbuzz_home_no_live.txt");
        let html = fs::read_to_string(fp).unwrap();

        let res_match_id_name: Vec<(String, String)> = vec![];
        let mut match_id_name = vec![];

        parse_all_live_matches_id_and_short_name(&html, &mut match_id_name);

        assert_eq!(res_match_id_name, match_id_name);
    }

    #[test]
    fn test_parse_scorecard_one_innings() {
        let fp = format!(
            "{}{}",
            TEST_FILES_PATH, "cricbuzz_scorecard_one_innings.txt"
        );
        let html = fs::read_to_string(fp).unwrap();

        let mut scorecard = vec![];

        parse_scorecard(&html, &mut scorecard);

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path(SNAPSHOTS_PATH);

        settings.bind(|| {
            insta::assert_debug_snapshot!(scorecard);
        });
    }

    #[test]
    fn test_parse_scorecard_two_innings() {
        let fp = format!(
            "{}{}",
            TEST_FILES_PATH, "cricbuzz_scorecard_two_innings.txt"
        );
        let html = fs::read_to_string(fp).unwrap();

        let mut scorecard = vec![];

        parse_scorecard(&html, &mut scorecard);

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path(SNAPSHOTS_PATH);

        settings.bind(|| {
            insta::assert_debug_snapshot!(scorecard);
        });
    }

    #[test]
    fn test_parse_scorecard_three_innings() {
        let fp = format!(
            "{}{}",
            TEST_FILES_PATH, "cricbuzz_scorecard_three_innings.txt"
        );
        let html = fs::read_to_string(fp).unwrap();

        let mut scorecard = vec![];

        parse_scorecard(&html, &mut scorecard);

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path(SNAPSHOTS_PATH);

        settings.bind(|| {
            insta::assert_debug_snapshot!(scorecard);
        });
    }

    #[test]
    fn test_parse_scorecard_four_innings() {
        let fp = format!(
            "{}{}",
            TEST_FILES_PATH, "cricbuzz_scorecard_four_innings.txt"
        );
        let html = fs::read_to_string(fp).unwrap();

        let mut scorecard = vec![];

        parse_scorecard(&html, &mut scorecard);

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path(SNAPSHOTS_PATH);

        settings.bind(|| {
            insta::assert_debug_snapshot!(scorecard);
        });
    }
}
