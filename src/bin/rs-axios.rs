use reqwest;
use reqwest::Client;
use tokio;



#[tokio::main]
async fn main() {
    let url = "http://localhost:3000/get";
    match reqwest::get(url).await {
        Ok(resp) => {
            println!("Response Status: {}", resp.status());
            // 处理响应数据
            println!("{:#?}",resp.text().await.unwrap());
        }
        Err(err) => println!("Error: {}", err),
    }
}
