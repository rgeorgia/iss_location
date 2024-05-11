use error_chain::error_chain;
use serde_derive::Deserialize;
use chrono::{NaiveDateTime};
use std::io;
use std::io::Write; // <--- bring flush() into scope

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
    for count in 1..11 {
        let res = reqwest::get("http://api.open-notify.org/iss-now.json").await?;
        let response = res.json::<IssPosition>().await?;

        print!("Long: {}\t", response.iss_position.longitude);
        print!("Lat: {}\t", response.iss_position.latitude);
        print!("Time: {:?}\n", NaiveDateTime::from_timestamp_opt(response.timestamp, 0).unwrap());
        io::stdout().flush().unwrap();
    }
    Ok(())
}
