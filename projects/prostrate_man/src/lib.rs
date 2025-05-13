use reqwest::Client;
use serde_json::Value;
use std::error::Error;

pub struct Config {
    pub client: Client,
    pub req_type: String,
    pub url: String,
    pub req: Value,
    pub bearer: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, Box<dyn Error>> {
        if args.len() < 4 {
            return Err("not enough arguments".into());
        }

        let client = reqwest::Client::new();

        let req_type: String = args[1].clone();
        let url: String = args[2].clone();
        let req: serde_json::Value = serde_json::from_str(&args[3].clone()).unwrap();
        // let req: Value = serde_json::from_str(&args[3].clone()).map_err(Box::new)?;
        let bearer: String = if args.len() > 4 {
            args[4].clone()
        } else {
            String::from("")
        };
        Ok(Config {
            client,
            req_type,
            url,
            req,
            bearer,
        })
    }
}

pub async fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let res = match config.req_type.as_str() {
        "get" => get(config).await?,
        "post" => post(config).await?,
        _ => serde_json::from_str(r#"{"error":"your response type isnt valid"}"#)
            .map_err(Box::new)?,
    };
    let skib: String = res["token"].to_string();
    println!("{}", skib);
    println!("{:#?}", res);
    Ok(())
}

async fn get(config: Config) -> Result<Value, Box<dyn Error>> {
    let res = config
        .client
        .get(config.url)
        .send()
        .await?
        .json::<Value>()
        .await?;
    Ok(res)
}
async fn post(config: Config) -> Result<Value, Box<dyn Error>> {
    let res = config
        .client
        .post(config.url)
        .json(&config.req)
        .send()
        .await?
        .json::<Value>()
        .await?;
    Ok(res)
}
