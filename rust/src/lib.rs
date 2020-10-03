mod types;
use anyhow::Error;
/// term -> response
/// response -> xml
/// xml -> id's
/// ids -> responses
/// response -> methods and materials
use reqwest::{Client, Response};
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

pub async fn search_response_to_result(r: Response) -> Result<SearchResult, Error> {
    let text = r.text().await?;
    let response: SearchResponse = serde_json::from_str(&text)?;
    Ok(response.e_search_result)
}
