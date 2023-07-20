use std::env;

use std::fs::File;
use std::io::{BufRead, BufReader};

use reqwest;

use colored::*;

async fn discover(url: String, wordlist: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let file = File::open(wordlist).expect("Failed to open the file.");

    // Create a BufReader to read the file efficiently
    let reader = BufReader::new(file);

    // Iterate over each line in the wordlist
    for line in reader.lines() {
        if let Ok(word) = line {
            let target = url.clone().to_string() + &*word.to_string();
            let response = reqwest::get(target.clone()).await?;
            let status = response.status().to_string();
            //let resp = response.json::<HashMap<String, String>>().await?;

            let code:Option<&str> = status.split_whitespace().nth(0);
            let result = match code {
                Some(code) => code.to_string(),
                None => String::from("Unknown"),
            };
            println!("{} {} {}", result.green(), word, target.clone().magenta());
        }
    }
    Ok(())
}

async fn discover_extract(url: String, wordlist: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let file = File::open(wordlist).expect("Failed to open the file.");

    // Create a BufReader to read the file efficiently
    let reader = BufReader::new(file);

    // Iterate over each line in the wordlist
    for line in reader.lines() {
        if let Ok(word) = line {
            let target = url.clone().to_string() + &*word.to_string();
            let response = reqwest::get(target.clone()).await?;
            let status = response.status().to_string();
            //let resp = response.json::<HashMap<String, String>>().await?;

            let code:Option<&str> = status.split_whitespace().nth(0);
            let result = match code {
                Some(code) => code.to_string(),
                None => String::from("Unknown"),
            };

            if result == "200" {
                println!("{} {} {}", result.green(), word, target.clone().magenta());
            } else {
                println!("Why");
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    //monologue0 --url1 https://discord.com/2 --wordlist3 test.txt4  --extract5
    if args.len() > 4 {
        let argument = &args[1].to_string();
        match argument.as_str() {
            "--url" => {
                let url = &args[2].to_string();
                let wordlist = &args[4].to_string();
                if args.len() > 6 {
                    let extractor = &args[5].to_string();
                    if extractor == "--extract" {
                        let _ = tokio::task::spawn(discover_extract(url.clone(),wordlist.clone())).await;
                    } else {
                        println!("Unknown")
                    }
                }

                let _ = tokio::task::spawn(discover(url.clone(), wordlist.clone())).await;
            },

            "help" => {
                println!("--help for help");
                println!("usage: --url [website] --wordlist [wordlist] [optional]");
            },
            _  => println!("Unknown Argument. Please --help"),
        }
    } else {
        println!("No argument provided");
    }
}
