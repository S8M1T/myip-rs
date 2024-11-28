use colored::*;
use reqwest::{Client, Error};
use serde::Deserialize;
use std::time::Duration;

#[derive(Deserialize, Debug)]
struct IpInfo {
    #[serde(rename = "YourFuckingIPAddress")]
    ip_address: String,
    #[serde(rename = "YourFuckingLocation")]
    location: String,
    #[serde(rename = "YourFuckingHostname")]
    hostname: String,
    #[serde(rename = "YourFuckingISP")]
    isp: String,
    #[serde(rename = "YourFuckingTorExit")]
    tor_exit: bool,
}

#[tokio::main]
async fn main() {
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("Failed to build HTTP client");

    let url = "https://ipv4.json.myip.wtf";

    match fetch_ip_info(&client, url).await {
        Ok(ip_info) => display_ip_info(ip_info),
        Err(e) => eprintln!("{}", format!("Failed to fetch IP details: {}", e).red()),
    }
}

async fn fetch_ip_info(client: &Client, url: &str) -> Result<IpInfo, Error> {
    let response = client
        .get(url)
        .header(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.3",
        )
        .send()
        .await?
        .error_for_status()?;

    let ip_info = response.json::<IpInfo>().await?;
    Ok(ip_info)
}

fn display_ip_info(ip_info: IpInfo) {
    println!("{}", "Your IP Details:\n".green().bold());
    println!("IP Address   : {}", ip_info.ip_address.yellow());
    println!("Location     : {}", ip_info.location.cyan());
    println!("Hostname     : {}", ip_info.hostname.magenta());
    println!("ISP          : {}", ip_info.isp.red());
    println!("Tor Exit Node: {}", ip_info.tor_exit.to_string().blue());
}
