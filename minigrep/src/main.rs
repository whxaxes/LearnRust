use std::{env, process};
use minigrep::{Config, run};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("parse arguments failed with reason: {}", err);
        process::exit(1);
    });

    let result = run(config);
    if let Err(e) = result {
        println!("execute failed with {:?}", e)
    } else if let Ok(str) = result {
        println!("{}", str);
    }
}
