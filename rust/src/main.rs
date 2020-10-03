use anyhow::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let term = "breast+cancer";
    let response = rust::pubmed_search(term).await?;
    eprintln!("Status: {}, URL: {}", response.status(), response.url());
    eprintln!("Headers:\n{:#?}", response.headers());

    let body = response.text().await?;
    println!("{}", body);

    Ok(())
}
