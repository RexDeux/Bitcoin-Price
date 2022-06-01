use serde::{Deserialize,Serialize};
use reqwest;
use std::{env, fs, io::Write};
use exitfailure::ExitFailure;
use reqwest::Url;
use structopt::StructOpt;
use chrono::{Local, Utc, TimeZone};
use std::time::Duration;
use tokio::{task, time}; // 1.3.0


#[derive(StructOpt)]
struct Cli {
    bitcoin: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub bitcoin: Bitcoin,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bitcoin {
    pub usd: i64,
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let date = chrono::offset::Local::now();
    let args = Cli::from_args();
    let response = Root::get(&args.bitcoin).await?;
    let price = response.bitcoin.usd;
    let mut crypto = fs::File::create("bitcoin_price.json").expect("Failed to create");
    
    let msg = format!(
        "Right now, {}, for {} the price is {} ", 
        date, args.bitcoin, price
    );
    
    let forever = task::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(10));



        for n in 1..100 {
            if price > 30000{
                let n = format!(
                    "Right now, {}, for {} the price is {} ", 
                    date, args.bitcoin, price
                );
                interval.tick().await;
                crypto.write_all(msg.as_bytes());
                println!(
                    "Right now ,{}, for {} the price is :{} ", 
                    date, args.bitcoin, price
                );}
            else {
                println!("Bitcoin has plunged below 3000")
            }
        }
    });

    forever.await;

    Ok(())

}
impl Root {
    async fn get(crypto: &String) -> Result<Self, ExitFailure> {
        let url: String = format!(
            "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=USD",
            crypto,
        );
        let url: Url = Url::parse(&*url)?;

        let resp = reqwest::get(url).await?.json::<Root>().await?;
        Ok(resp)
    }
}
