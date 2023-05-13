use reqwest;
use tokio;
fn main() {
    println!("Hello, world!");
    get_request();
}
#[tokio::main]
async fn get_request() {
    let body: Result<reqwest::Response, reqwest::Error> = reqwest::get("https://www.wired.com/feed")
        .await;
    match body {
        Ok(resp) => {
            println!("Response: {:?}", resp);
            println!("Headers:\n{:?}", resp.headers());
            println!("Status: {:?}", resp.status());
            println!("Body:\n{}", resp.text().await.unwrap());
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}