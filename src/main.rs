use scraper::{Html, Selector};
use serde::Deserialize;
use std::io::{self, Write};
use std::vec::Vec;
use std::{thread, time};

pub fn get_authors(document: &Html) -> String {
    //<meta name="author" content="Nailia Bagirova, Nvard Hovhannisyan"/>
    let selector = Selector::parse(r#"meta[name="author"]"#).unwrap();
    let mut selection = document.select(&selector);
    let element_ref = selection.next().unwrap();
    let content = element_ref.value().attr("content").unwrap();
    content.to_string()
}

pub fn get_keywords(document: &Html) -> Vec<String> {
    // Get keywords from a reuters article document
    //<meta name="keywords" content="UK,ARMENIA...">
    let selector = Selector::parse(r#"meta[name="keywords"]"#).unwrap();
    let mut selection = document.select(&selector);
    let element_ref = selection.next().unwrap();
    let content = element_ref.value().attr("content").unwrap();
    let keywords = content
        .split(',')
        .map(|keyword| keyword.to_string())
        .collect::<Vec<_>>();
    keywords
}

pub fn get_paragraphs(document: &Html) -> Vec<String> {
    //<p class="Paragraph-paragraph-2Bgue ArticleBody-para-TD_9x">Get this text</p>
    let i_selector =
        Selector::parse(r#"p[class="Paragraph-paragraph-2Bgue ArticleBody-para-TD_9x"]"#).unwrap();

    let element_refs = document.select(&i_selector).collect::<Vec<_>>();
    let paragraphs: Vec<String> = element_refs
        .iter()
        .map(|element_ref| element_ref.text().collect())
        .collect();
    paragraphs
}

struct Article<'a> {
    headline: &'a Headline,
    paragraphs: Vec<String>,
}

fn make_article(raw_text: String, headline: &Headline) -> Article {
    let document = Html::parse_document(&raw_text);
    let paragraphs = get_paragraphs(&document);
    Article {
        headline,
        paragraphs,
    }
}

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
    /// The main wire response has a key "headlines"
    /// that holds a list of of Headline
    headlines: Vec<Headline>,
}

impl Headline {
    fn build_url(&self) -> String {
        // Build the URL by concatenating the base with the article ref
        let base = String::from("https://www.reuters.com");
        let ending = String::from(&self.url);
        base + &ending
    }

    async fn download_article(&self) -> String {
        let url = self.build_url();
        //println!("Downloading {:#?}", url);
        reqwest::get(&url).await.unwrap().text().await.unwrap()
    }
}

async fn get_headlines() -> Result<Vec<Headline>, Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://www.reuters.com/assets/jsonWireNews")
        .await?
        .json::<Wire>()
        .await?;
    Ok(resp.headlines)
}

impl<'a> Article<'a> {
    fn to_text(&self) -> String {
        let sep =
            "\n\n################################################################################"
                .to_string();
        let joined_paragraphs = self.paragraphs.join("\n\n");
        let text = vec![sep, self.headline.headline.clone(), joined_paragraphs].join("\n\n");
        textwrap::fill(&text, 80)
    }
}

fn ttype_print(article: &Article) {
    let sleep_delay = time::Duration::from_millis(25);
    let text = article.to_text();
    for c in text.chars() {
        print!("{}", c);
        thread::sleep(sleep_delay);
        io::stdout().flush().unwrap();
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let headlines = get_headlines().await.unwrap();

    for headline in headlines.iter() {
        let article = make_article(headline.download_article().await, headline);
        ttype_print(&article);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{get_authors, get_keywords, get_paragraphs};
    use scraper::Html;
    #[test]
    fn test_get_keywords() {
        let raw_html = include_str!("../tests/data/article.html");
        let document = Html::parse_document(raw_html);
        let keywords = get_keywords(&document);

        assert!(keywords.contains(&"Azerbaijan".to_string()));
        assert!(keywords.contains(&"Diplomacy / Foreign Policy".to_string()));
    }

    #[test]
    fn test_get_paragraphs() {
        let raw_html = include_str!("../tests/data/article.html");
        let document = Html::parse_document(raw_html);
        let paragraphs = get_paragraphs(&document);

        // TODO: Move to a .txt and include_str!
        assert!(paragraphs.contains(&"BAKU/YEREVAN (Reuters) - Armenia and Azerbaijan said they had agreed on Saturday to a new humanitarian ceasefire from midnight (2000 GMT) in fighting over Azerbaijan’s ethnic Armenian-controlled enclave of Nagorno-Karabakh.".to_string()));
    }

    #[test]
    fn test_get_authors() {
        let raw_html = include_str!("../tests/data/article.html");
        let document = Html::parse_document(raw_html);
        let authors = get_authors(&document);

        assert!(authors == "Nailia Bagirova, Nvard Hovhannisyan".to_string());
    }

    #[test]
    fn test_get_headline() {
        // let raw_html = include_str!("../tests/data/article.html");
        // let document = Html::parse_document(raw_html);

        //TODO: Implement headline
        //let headline = get_headline(&document);
        //assert!(headline == "Armenia and Azerbaijan agree new Nagorno-Karabakh ceasefire".to_string());
    }
}
