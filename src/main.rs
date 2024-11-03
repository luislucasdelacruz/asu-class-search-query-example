#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let terms_resp = fetch_terms().await?;
    println!("{terms_resp:#?}");

    Ok(())
}

async fn fetch_terms() -> Result<serde_json::Value, reqwest::Error> {
    let client = reqwest::Client::new();
    client
        .get(
            "https://eadvs-cscc-catalog-api.apps.asu.edu/catalog-microservices/api/v1/search/terms",
        )
        .header(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/124.0",
        )
        .header("Accept", "*/*")
        .header("Accept-Encoding", "gzip, deflate, br")
        .header("Referer", "https://catalog.apps.asu.edu/")
        .header("authorization", "Bearer null")
        .header("Origin", "https://catalog.apps.asu.edu/")
        .header("DNT", "1")
        .send()
        .await?
        .json()
        .await
}
