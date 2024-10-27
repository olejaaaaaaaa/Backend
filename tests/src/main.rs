use reqwest::*;
use tokio::*;

#[tokio::main]
async fn main() {
    let resp = reqwest::get("http://127.0.0.1:8080/get_book?id=0").await.unwrap();
    println!("{:?}", resp.text().await);

    let resp = reqwest::get("http://127.0.0.1:8080/get_books").await.unwrap();
    println!("{:?}", resp.text().await);
}
