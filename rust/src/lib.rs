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

pub async fn pubmed_query(query: &str) -> Result<Response, Error> {
    let client = Client::new();
    let query = [("db", "pubmed"), ("term", query)];
    let base_url = BASE_URL.to_string() + "esearch.fcgi";

    client
        .get(&base_url)
        .query(&query)
        .send()
        .await
        .map_err(|e| e.into())
}
