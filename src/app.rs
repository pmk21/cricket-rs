use select::document::Document;
use select::predicate::{Class, Name, Predicate};

use crate::cricbuzz_api::*;

pub struct MatchInfo {
    pub match_short_name: String,
    pub cricbuzz_match_id: u32,
    pub cricbuzz_match_api_link: String,
    pub cricbuzz_info: CricbuzzJson,
}

pub struct App {
    pub matches_info: Vec<MatchInfo>,
    pub focused_tab: u32,
}

const CRICBUZZ_URL: &str = "https://www.cricbuzz.com";
const CRICBUZZ_MATCH_API: &str = "https://www.cricbuzz.com/api/cricket-match/commentary/";

impl App {
    pub async fn new() -> App {
        let mut matches_info = vec![];
        let mut match_name_id: Vec<(String, String)> = vec![];
        match get_all_live_matches_id_and_short_name().await {
            Ok(v) => match_name_id = v,
            Err(e) => {
                println!("{:?}", e);
            }
        };

        println!("{:?}", match_name_id);

        for (name, id) in &match_name_id {
            let match_id: u32 = id.parse().unwrap();
            if let Ok(json) = get_match_info_from_id(match_id).await {
                matches_info.push(MatchInfo::new(
                    name.to_string(),
                    match_id,
                    format!("{}{}", String::from(CRICBUZZ_MATCH_API), match_id),
                    json,
                ));
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
    ) -> MatchInfo {
        MatchInfo {
            match_short_name,
            cricbuzz_match_id,
            cricbuzz_match_api_link,
            cricbuzz_info,
        }
    }
}

async fn get_all_live_matches_id_and_short_name(
) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    let resp_html = reqwest::get(CRICBUZZ_URL).await?.text().await?;

    let document = Document::from(resp_html.as_str());
    let mut match_id_name = vec![];

    for node in document.find(Class("cb-mat-mnu").descendant(Name("a"))) {
        // This check might break in the future
        if !node.text().is_empty() && !node.text().eq("MATCHES") {
            match node.attr("href") {
                Some(link) => {
                    let split_str: Vec<&str> = link.split('/').collect();
                    match_id_name.push((node.text(), String::from(split_str[2])));
                }
                _ => {}
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

    // let contents = fs::read_to_string("test.json").expect("Something went wrong");
    // let res: CricbuzzJson = serde_json::from_str(&contents).unwrap();
    let res: CricbuzzJson = serde_json::from_str(&resp).unwrap();
    // println!("page: {:#?}", res);
    Ok(res)
}
