use std::collections::HashMap;
use std::env;

use std::fs::File;
use std::io::{BufRead, BufReader};

use reqwest;
use tokio::task::JoinHandle;

use colored::*;

async fn discover(url: String, wordlist: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let file = File::open(wordlist).expect("Failed to open the file.");

    // Create a BufReader to read the file efficiently
    let reader = BufReader::new(file);

    // Iterate over each line in the wordlist
    for line in reader.lines() {
        if let Ok(word) = line {
            let target = url.clone().to_string() + &*word.to_string();
            let response = reqwest::get(target).await?;
            let status = response.status().to_string();
            let resp = response.json::<HashMap<String, String>>().await?;

            let code = status.split_whitespace().nth(2);
            println!("{:?}", code.expect("Failure").blue());
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    //mono[0 --url1 https://example.com/2 --3 blah.txt4
    if args.len() > 2 {
        let argument = &args[1].to_string();
        match argument.as_str() {
            "--url" => {
                let url = &args[2].to_string();
                let wordlist = &args[4].to_string();
                let _ = tokio::task::spawn(discover(url.clone(), wordlist.clone())).await;
            },
            _  => println!("Unknown Argument. Please --help"),
        }
    } else {
        println!("No argument provided");
    }
}
