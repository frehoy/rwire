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

impl Headline {

    fn build_url(&self) -> String {
        // Build the URL by concatenating the base with the article ref
        let base = String::from("https://www.reuters.com");
        let ending = String::from(&self.url);
        let url = base + &ending;
        url
    }

    async fn download_article(&self) {
        let url = self.build_url();
        println!("Downloading {:#?}", url);
        let resp = reqwest::get(&url).await.unwrap();
        println!("{:#?}", resp.text().await);
    }
} 
#[derive(Deserialize, std::fmt::Debug)]
struct Wire {
    /// The main wire response has a key "headlines"
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
    println!("Got {:#?} headlines", headlines.len());

    for headline in headlines.iter() {
        headline.download_article().await;
    }

    Ok(())
}
