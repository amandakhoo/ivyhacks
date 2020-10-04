use anyhow::Error;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "pubmed fetcher",
    about = "A tool to download xml papers from pubmed"
)]
struct Args {
    /// Activate debug mode for manually checking downloaded xml papers.
    #[structopt(short, long)]
    debug: bool,

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
        debug,
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
    if debug {
        std::fs::write("debug.xml", &body)?;
    }
    let doc = rust::response_to_xml(&body)?;
    let method_paragraphs = rust::method_paragraphs(&doc);
    for paragraph in method_paragraphs {
        if let Some(paragraph) = paragraph {
            println!("{}", paragraph);
        } else {
            eprintln!("------------paper either wasn't full text or had an unexpected format");
        }
    }

    Ok(())
}
