use reqwest::Client;
use scraper::{ElementRef, Html, Selector};

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

#[derive(Default)]
pub struct App {
    req_clt: Client,
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
        let req_clt = Client::new();

        match get_all_live_matches_id_and_short_name(&req_clt).await {
            Ok(v) => match_name_id = v,
            Err(e) => {
                println!("{:?}", e);
            }
        };

        for (name, id) in &match_name_id {
            let match_id: u32 = id.parse().unwrap();
            if let Ok(json) = get_match_info_from_id(&req_clt, match_id).await {
                if prepare_scorecard(&req_clt, match_id, &mut scorecard)
                    .await
                    .is_ok()
                {
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
            req_clt,
            matches_info,
            focused_tab,
        }
    }

    pub async fn update_on_tick(&mut self) {
        // WARN: Update algorithm might be slow for larger number of matches
        // For now this should be fine and not cause any bottlenecks
        let mut match_name_id: Vec<(String, String)> = vec![];
        let mut scorecard: Vec<MatchInningsInfo> = vec![];

        match get_all_live_matches_id_and_short_name(&self.req_clt).await {
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
                if let Ok(json) = get_match_info_from_id(&self.req_clt, mid).await {
                    if prepare_scorecard(&self.req_clt, mid, &mut scorecard)
                        .await
                        .is_ok()
                    {
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
    req_clt: &Client,
) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    let resp_html = req_clt.get(CRICBUZZ_URL).send().await?.text().await?;
    let mut match_id_name = vec![];

    parse_all_live_matches_id_and_short_name(&resp_html, &mut match_id_name);

    Ok(match_id_name)
}

fn parse_all_live_matches_id_and_short_name(html: &str, match_id_name: &mut Vec<(String, String)>) {
    let doc = Html::parse_document(html);
    let nav_sel = Selector::parse("nav.cb-mat-mnu").unwrap();
    let sel_a = Selector::parse("a").unwrap();

    if let Some(nav) = doc.select(&nav_sel).next() {
        for link in nav.select(&sel_a) {
            let text = link
                .text()
                .collect::<Vec<&str>>()
                .concat()
                .trim()
                .to_string();
            if !text.is_empty() && !text.eq("MATCHES") {
                if let Some(spl) = text.split('-').nth(1) {
                    if spl.trim().eq("Live") {
                        if let Some(href) = link.value().attr("href") {
                            let split_str = href.split('/').collect::<Vec<&str>>();
                            match_id_name.push((text, split_str[2].to_string()));
                        }
                    }
                }
            }
        }
    }
}

async fn get_match_info_from_id(
    req_clt: &Client,
    match_id: u32,
) -> Result<CricbuzzJson, Box<dyn std::error::Error>> {
    let resp = req_clt
        .get(format!("{}{}", String::from(CRICBUZZ_MATCH_API), match_id))
        .send()
        .await?
        .text()
        .await?;

    let res: CricbuzzJson = serde_json::from_str(&resp).unwrap();
    Ok(res)
}

async fn prepare_scorecard(
    req_clt: &Client,
    id: u32,
    scorecard: &mut Vec<MatchInningsInfo>,
) -> Result<(), Box<dyn std::error::Error>> {
    let resp_html = req_clt
        .get(format!(
            "{}{}",
            String::from(CRICBUZZ_MATCH_SCORECARD_API),
            id
        ))
        .send()
        .await?
        .text()
        .await?;

    parse_scorecard(&resp_html, scorecard);

    Ok(())
}

fn parse_scorecard(html: &str, scorecard: &mut Vec<MatchInningsInfo>) {
    let doc = Html::parse_document(html);

    for ino in 1..5 {
        let inngs_sel = Selector::parse(format!("div[id=\"innings_{}\"]", ino).as_str()).unwrap();
        if let Some(div) = doc.select(&inngs_sel).next() {
            populate_innings_info(&div, scorecard);
        }
    }
}

fn populate_innings_info(div: &ElementRef, scorecard: &mut Vec<MatchInningsInfo>) {
    // This unwrap will probably never panic
    let sel_scrd_items = Selector::parse("div.cb-scrd-itms").unwrap();
    let sel_div = Selector::parse("div").unwrap();

    let mut match_inngs_info = MatchInningsInfo::default();

    for inner_div in div.select(&sel_scrd_items) {
        // Check for batsman or bowler scorcard info
        let num_child_div = inner_div.select(&sel_div).count();
        if num_child_div == 7 {
            // This is for batsman info
            let mut bat_info = BatsmanInfo::default();
            let mut divs = inner_div.select(&sel_div);
            if let Some(bat_name_link) = divs.next() {
                bat_info.name = bat_name_link
                    .text()
                    .collect::<Vec<&str>>()
                    .concat()
                    .trim()
                    .to_string();
            }

            bat_info.status = divs
                .next()
                .unwrap()
                .text()
                .collect::<Vec<&str>>()
                .concat()
                .trim()
                .to_string();
            bat_info.runs = divs.next().unwrap().inner_html().trim().to_string();
            bat_info.balls = divs.next().unwrap().inner_html().trim().to_string();
            bat_info.fours = divs.next().unwrap().inner_html().trim().to_string();
            bat_info.sixes = divs.next().unwrap().inner_html().trim().to_string();
            bat_info.strike_rate = divs.next().unwrap().inner_html().trim().to_string();

            match_inngs_info.batsman_details.push(bat_info);
        } else if num_child_div == 8 {
            // This is for bowler info
            let mut bowl_info = BowlerInfo::default();
            let mut divs = inner_div.select(&sel_div);
            if let Some(bowl_name_link) = divs.next() {
                bowl_info.name = bowl_name_link
                    .text()
                    .collect::<Vec<&str>>()
                    .concat()
                    .trim()
                    .to_string();
            }

            bowl_info.overs = divs.next().unwrap().inner_html().trim().to_string();
            bowl_info.maidens = divs.next().unwrap().inner_html().trim().to_string();
            bowl_info.runs = divs.next().unwrap().inner_html().trim().to_string();
            bowl_info.wickets = divs.next().unwrap().inner_html().trim().to_string();
            bowl_info.no_balls = divs.next().unwrap().inner_html().trim().to_string();
            bowl_info.wides = divs.next().unwrap().inner_html().trim().to_string();
            bowl_info.economy = divs.next().unwrap().inner_html().trim().to_string();

            match_inngs_info.bowler_details.push(bowl_info);
        }
    }
    scorecard.push(match_inngs_info);
}

#[cfg(test)]
pub fn parse_scorecard_from_file(file: &str, scorecard: &mut Vec<MatchInningsInfo>) {
    parse_scorecard(file, scorecard);
}

#[cfg(test)]
pub fn create_match_info(
    match_short_name: String,
    cricbuzz_match_id: u32,
    cricbuzz_match_api_link: String,
    cricbuzz_info: CricbuzzJson,
    scorecard: Vec<MatchInningsInfo>,
) -> MatchInfo {
    MatchInfo::new(
        match_short_name,
        cricbuzz_match_id,
        cricbuzz_match_api_link,
        cricbuzz_info,
        scorecard,
    )
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
