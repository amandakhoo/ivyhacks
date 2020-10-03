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
        ("db", "pubmed"),
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
        ("db", "pubmed"),
        ("query_key", &query_key.to_string()),
        ("usehistory", "y"),
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
    assert_eq!(doc.descendants()
            .find(|node| node.attribute("NlmCategory") == Some("METHODS"))
            .and_then(|node| node.text()),
            Some("The CBM records thermodynamic metabolic data from the breast skin surface over a period of time using two wearable biometric patches consisting of eight sensors each and a data recording device. The acquired multi-dimensional temperature time series data are analyzed to determine the presence of breast tissue abnormalities. The objective of this paper is to present the scientific background of CBM and also to describe the history around the design and development of the technology.")
    );
    Ok(())
}
