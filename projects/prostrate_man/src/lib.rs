use serde_json::Value;
use std::error::Error;

pub struct Config {
    pub req_type: String,
    pub url: String,
    pub req: serde_json::Value,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, Box<dyn Error>> {
        if args.len() < 4 {
            return Err("not enough arguments".into());
        }

        let req_type: String = args[1].clone();
        let url: String = args[2].clone();
        let req: Value = serde_json::from_str(&args[3].clone()).map_err(Box::new)?;
        Ok(Config { req_type, url, req })
    }
}

pub async fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let res = match config.req_type.as_str() {
        "get" => get(config).await?,
        _ => serde_json::from_str(r#"{"error":"your response type isnt valid"}"#).map_err(Box::new)?,
    };
    println!("{res:#?}");
    Ok(())
}

async fn get(config: Config) -> Result<Value, Box<dyn Error>> {
    let res = reqwest::get(config.url).await?.json::<Value>().await?;
    Ok(res)
}
