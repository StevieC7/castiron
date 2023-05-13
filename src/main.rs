use reqwest;
use tokio;
fn main() {
    println!("Hello, world!");
    get_request();
}
#[tokio::main]
async fn get_request() {
    let result: Result<reqwest::Response, reqwest::Error> = reqwest::get("https://www.wired.com/feed")
        .await;
    match result {
        Ok(resp) => {
            println!("Response: {:?}", resp);
            println!("Headers:\n{:?}", resp.headers());
            println!("Status: {:?}", resp.status());
            let text = resp.text().await;
            match text {
                Ok(val) => {
                    println!("Body:\n{}", val)
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            }
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}