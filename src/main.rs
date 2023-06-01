use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client
        .get("https://api.ipify.org/?format=text")
        .send()
        .await?
        .text()
        .await?;

    let location = response.trim();

    if !location.is_empty() {
        println!("Location: {}", location);
    } else {
        println!("Location data not available");
    }

    Ok(())
}
