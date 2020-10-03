use anyhow::Error;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "pubmed fetcher",
    about = "A tool to download xml papers from pubmed"
)]
struct Args {
    /// Terms for which to search.
    ///
    /// These must be separated with `+`'s, eg, breast+cancer.
    #[structopt(name = "TERMS")]
    terms: String,

    /// Maximum number of articles to return
    #[structopt(short, long, default_value = "20")]
    max: usize,

    /// Number of article to start search at.
    /// This can be if you want to iteratively download sets of 20 articles
    #[structopt(short, long, default_value = "0")]
    start: usize,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let Args {
        terms: term,
        max: retmax,
        start: retstart,
    } = Args::from_args();
    let response = rust::pubmed_search(&term).await?;
    eprintln!("Status: {}, URL: {}", response.status(), response.url());
    eprintln!("Headers:\n{:#?}", response.headers());

    let response = rust::search_response_to_result(response).await?;

    let fetch_response =
        rust::pubmed_fetch(response.querykey, &response.webenv, retmax, retstart).await?;
    let body = fetch_response.text().await?;
    println!("{}", body);

    Ok(())
}
