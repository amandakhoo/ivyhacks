mod types;
use anyhow::Error;
/// term -> response
/// response -> xml
/// xml -> id's
/// ids -> responses
/// response -> methods and materials
use reqwest::{Client, Response};
use roxmltree::Document;
use types::*;

const BASE_URL: &str = "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/";

pub async fn pubmed_search(query: &str) -> Result<Response, Error> {
    let client = Client::new();
    let query = [
        ("db", "pmc"),
        ("term", query),
        ("usehistory", "y"),
        ("retmode", "json"),
    ];
    let base_url = BASE_URL.to_string() + "esearch.fcgi";

    client
        .get(&base_url)
        .query(&query)
        .send()
        .await
        .map_err(|e| e.into())
}

/// maybe just need webenv?
/// ie: https://eutils.ncbi.nlm.nih.gov/entrez/eutils/efetch.fcgi?db=pubmed&query_key=1&usehistory=y&WebEnv=MCID_5f77bdfcb48029616024b3fb&retmode=xml&retmax=100&retstart=18
pub async fn pubmed_fetch(
    query_key: usize,
    web_env: &str,
    retmax: usize,
    retstart: usize,
) -> Result<Response, Error> {
    let client = Client::new();
    let query = [
        ("db", "pmc"),
        ("query_key", &query_key.to_string()),
        ("WebEnv", web_env),
        ("retmode", "xml"),
        ("retmax", &retmax.to_string()),
        ("retstart", &retstart.to_string()),
    ];
    let base_url = BASE_URL.to_string() + "efetch.fcgi";

    client
        .get(&base_url)
        .query(&query)
        .send()
        .await
        .map_err(|e| e.into())
}

/// Given a response whose body is xml, return the corresponding xml document
pub fn response_to_xml<'a>(s: &'a str) -> Result<Document<'a>, Error> {
    Document::parse(&s).map_err(|e| e.into())
}

pub async fn search_response_to_result(r: Response) -> Result<SearchResult, Error> {
    let text = r.text().await?;
    let response: SearchResponse = serde_json::from_str(&text)?;
    Ok(response.e_search_result)
}

#[test]
fn xml_tests() -> Result<(), Error> {
    let response = include_str!("../../data/test-fetch.xml");
    let doc = response_to_xml(response)?;
    let mut articles = doc
        .root()
        .children()
        .next()
        .unwrap()
        .children()
        .filter(|node| node.is_element());
    let third_article = articles.nth(3).unwrap();
    println!("{:?}", third_article);
    let article_body = third_article
        .children()
        .find(|node| node.tag_name().name() == "body");
    let methods_node = article_body.and_then(|node| {
        node.descendants()
            .find(|node| node.text() == Some("Methods"))
    });
    println!("{:?}", methods_node);
    let methods_section = methods_node.and_then(|node| node.parent());
    println!("{:?}", methods_section);
    let mut methods_paragraphs = methods_section
        .map(|node| {
            node.descendants()
                .filter(|node| node.tag_name().name() == "p")
        })
        .unwrap();

    assert_eq!(methods_paragraphs.next().and_then(|node| node.text()),
            Some("The present study was conducted at the Radiation Oncology Unit of the Careggi University Hospital of Florence, Italy. This is a prospective monocenter study including cancer patients admitted to the department to receive either systemic and/or radiation treatment. All patients recruited underwent a survey approved by our institutional ethics review board.")
    );
    Ok(())
}
