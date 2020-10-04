use anyhow::Error;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "pubmed fetcher",
    about = "A tool to download xml papers from pubmed"
)]
struct Args {
    /// Activate debug mode for manually checking downloaded xml papers.
    #[structopt(long)]
    debug: bool,

    /// Activate demo mode.
    ///
    /// This saves the results as well as the search terms to be used by the web app.
    /// In the future, this will be rendered obsolete by using core functionality
    /// within the web app directly.
    #[structopt(short, long)]
    demo: bool,

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
        demo,
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

    if demo {
        let search_results = rust::Search::new(
            &term,
            &method_paragraphs
                .iter()
                .filter_map(|result| result.as_deref())
                .collect::<Vec<_>>(),
        );
        let output = serde_json::to_string(&search_results)?;
        std::fs::write("demo.json", output)?;
    }
    for paragraph in method_paragraphs {
        if let Some(paragraph) = paragraph {
            println!("{}", paragraph);
        } else {
            eprintln!("------------paper either wasn't full text or had an unexpected format");
        }
    }

    Ok(())
}
