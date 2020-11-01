use serde::Deserialize;
use std::vec::Vec;

#[allow(non_snake_case)]
#[derive(Deserialize, std::fmt::Debug)]
struct Headline {
    /// The structure of each Headline as it comes from the API
    id: String,
    headline: String,
    dateMillis: String,
    formattedDate: String,
    url: String,
    mainPicUrl: String,
}
#[derive(Deserialize, std::fmt::Debug)]
struct Wire {
    /// The main wire response is a key "headlines"
    /// that holds a list of of Headline
    headlines: Vec<Headline>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://www.reuters.com/assets/jsonWireNews")
        .await?
        .json::<Wire>()
        .await?;
    let headlines = resp.headlines;
    println!("{:#?}", headlines);
    Ok(())
}
