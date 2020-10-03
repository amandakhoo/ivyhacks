use anyhow::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let term = "breast+cancer";
    let response = rust::pubmed_search(term).await?;
    eprintln!("Status: {}, URL: {}", response.status(), response.url());
    eprintln!("Headers:\n{:#?}", response.headers());

    let response = rust::search_response_to_result(response).await?;

    let fetch_response = rust::pubmed_fetch(response.querykey, &response.webenv, 100, 0).await?;
    let body = fetch_response.text().await?;
    println!("{}", body);

    Ok(())
}
