//discover.rs
use std::fs::File;
use std::io::{BufRead, BufReader};
use reqwest;
use colored::*;

pub async fn discover(url: String, wordlist: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let file = File::open(wordlist).expect("Failed to open the file.");

    // Create a BufReader to read the file efficiently
    let reader = BufReader::new(file);

    // Iterate over each line in the wordlist
    for line in reader.lines() {
        if let Ok(word) = line {
            let target = url.clone().to_string() + &word.to_string();
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