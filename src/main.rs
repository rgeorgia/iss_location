use error_chain::error_chain;
use serde_derive::Deserialize;
use chrono::{NaiveDateTime};

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}
#[derive(Deserialize, Debug)]
struct Point {
    longitude: String,
    latitude: String,
}
#[derive(Deserialize, Debug)]
struct IssPosition {
    timestamp: i64,
    message: String,
    iss_position: Point,
}
#[tokio::main]
async fn main() -> Result<()> {
    let res = reqwest::get("http://api.open-notify.org/iss-now.json").await?;
    // println!("Status: {}", res.status());
    // println!("Headers:\n{:#?}", res.headers());

    let response = res.json::<IssPosition>().await?;

    println!("Message: {}", response.message);
    println!("Long: {}", response.iss_position.longitude);
    println!("Lat: {}", response.iss_position.latitude);
    println!("Time: {:?}", NaiveDateTime::from_timestamp_opt(response.timestamp, 0).unwrap());
    Ok(())
}
