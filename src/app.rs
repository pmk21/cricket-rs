use select::predicate::{Class, Name, Predicate};
use select::{document::Document, node::Node, predicate::Attr};

use crate::cricbuzz_api::*;

#[derive(Debug, Default, Clone)]
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

#[derive(Default, Debug, Clone)]
pub struct BatsmanInfo {
    pub name: String,
    pub status: String,
    pub runs: String,
    pub balls: String,
    pub fours: String,
    pub sixes: String,
    pub strike_rate: String,
}

#[derive(Debug, Default, Clone)]
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
    pub focused_tab: u32,
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
                if let Ok(_) = prepare_scorecard(match_id, &mut scorecard).await {
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

    let document = Document::from(resp_html.as_str());
    let mut match_id_name = vec![];

    for node in document.find(Class("cb-mat-mnu").descendant(Name("a"))) {
        // This check might break in the future
        if !node.text().is_empty() && !node.text().eq("MATCHES") {
            if node.text().split('-').nth(1).unwrap().trim().eq("Live") {
                match node.attr("href") {
                    Some(link) => {
                        let split_str: Vec<&str> = link.split('/').collect();
                        match_id_name.push((node.text(), String::from(split_str[2])));
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(match_id_name)
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

    let document = Document::from(resp_html.as_str());

    for i in 1..5 {
        if let Some(inngs) = document
            .find(Attr("id", format!("innings_{}", i.to_string()).as_ref()))
            .nth(0)
        {
            populate_innings_info(&inngs, scorecard);
        }
    }

    Ok(())
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
        if node.attr("class").is_some() {
            if node.children().count() == 15 {
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
