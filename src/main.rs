use std::fs;

mod cricbuzz_api;

use crate::cricbuzz_api::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let api = String::from("https://www.cricbuzz.com/api/cricket-match/commentary/35842");
    /*
    let resp = reqwest::get(api)
        .await?
        .json::<CricbuzzJson>()
        //.text()
        .await?;
    println!("{:#?}", resp);
    */
    let contents = fs::read_to_string("test.json").expect("Something went wrong");
    println!("{}", contents);
    let res: CricbuzzJson = serde_json::from_str(&contents).unwrap();

    println!("page: {:#?}", res);
    Ok(())
}
