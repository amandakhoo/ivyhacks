use anyhow::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let term = "breast+cancer";
    let response = rust::pubmed_query(term).await?;
    println!("Status: {}, URL: {}", response.status(), response.url());
    println!("Headers:\n{:#?}", response.headers());

    let body = response.text().await?;
    println!("Body: {}", body);

    Ok(())
}
