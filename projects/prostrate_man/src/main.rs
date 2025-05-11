use std::{env, process};

use prostrate_man::Config;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing args: {err}");
        process::exit(1);
    });


    if let Err(e) = prostrate_man::run(config).await {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
