mod parser;
mod utils;

use reqwest::Client;
use regex::Regex;
use chrono::{TimeZone, Utc};

#[tokio::main]
async fn main() {
    utils::banner();
    utils::copyright();

    let req = parser::initialize();

    match req.typex.as_ref() {
        "mail" => {
            proton_mail(&req.valuex).await.expect("Error while checking ProtonMail");
        },
        "ip" => {
            proton_vpn(&req.valuex).await.expect("Error while checking ProtonVPN");
        },
        _ => {
            println!("You need to choose at least 1 mode");
            std::process::exit(1)
        }
    }
}

async fn proton_mail(mail: &str) -> Result<(), reqwest::Error> {

    let client = Client::new();
    let url = format!("https://api.protonmail.ch/pks/lookup?op=index&search={}", mail);
    let url_key = format!("https://api.protonmail.ch/pks/lookup?op=get&search={}", mail);

    let response = client.get(url)
        .header("User-Agent", "ProSint")
        .send()
        .await?;

    let response_key = client.get(url_key)
        .header("User-Agent", "ProSint")
        .send()
        .await?;

    let body = response.text().await?;
    let body_key = response_key.text().await?;

    if body.contains("info:1:0") {
        println!("Bad mail")
    }
    else if body.contains("info:1:1") {

        let mut timestampd: i64 = 0;

        let regexptrn = vec![
            Regex::new(r"2048:(.*)::").unwrap(),
            Regex::new(r"4096:(.*)::").unwrap(),
            Regex::new(r"22::(.*)::").unwrap(),
        ];

        for regexp in regexptrn {
            if let Some(caps) = regexp.captures(&body) {
                if let Some(timestamp) = caps.get(1) {
                    let timestamp = timestamp.as_str();
                    timestampd= timestamp.parse().unwrap();
                }
            }
        }

        let dt = Utc.timestamp_opt(timestampd, 0).unwrap();

        println!("UID: {}", mail);
        println!("Creation Date: {}", dt);

    }
    else {
        println!("Can't query the AP");
    }
    println!("{}", body_key);

    Ok(())
}

async fn proton_vpn(ip: &str) -> Result<(), reqwest::Error> {

    let client = Client::new();
    let url = "https://api.protonmail.ch/vpn/logicals";

    let response = client.get(url)
        .header("User-Agent", "ProSint")
        .send()
        .await?;

    let body = response.text().await?;

    let ip = format!("{ip}\"");

    if body.contains(&ip) {
        println!("The ip is from ProtonVPN")
    }
    else {
        println!("This is not a ProtonVPN IP")
    }

    Ok(())
}
