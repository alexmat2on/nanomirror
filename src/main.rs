mod palette_extractor;

use nanoleaf_rs::NanoLeafClient;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    access_token: String,
    host: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("config.json")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let config: Config = serde_json::from_str(&contents).unwrap();
    let nanoleaf = NanoLeafClient::new(&config.access_token, &config.host);
    //let resp = nanoleaf.info().await?;
    //println!("{:#?}", resp);
    //let select = nanoleaf.select(Some("Cool Breeze".to_string())).await?;
    //println!("{:#?}", select.unwrap_or("None selected".to_string()));
    //let _ = nanoleaf.request_effect("ATEST2".to_string()).await?;
    //let _ = nanoleaf.add_wheel_effect("ATEST".to_string(), vec![lib::color::NanoLeafColor::new(50, 50, 50), lib::color::NanoLeafColor::new(89, 100, 85)]).await?;
    let _ = nanoleaf.on(true).await?;
    Ok(())
}
