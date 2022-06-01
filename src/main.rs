use serde::{Deserialize,Serialize};
use reqwest::{self, header::{AUTHORIZATION, CONTENT_TYPE, ACCEPT}};
use std::env;
use exitfailure::ExitFailure;
use reqwest::Url;
use structopt::StructOpt;

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
    let args = Cli::from_args();
    let response = Root::get(&args.bitcoin).await?;
    let price = response.bitcoin.usd;
    println!(
        "Right now for {} the price is {} ",
        args.bitcoin, response.bitcoin.usd
    );

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
