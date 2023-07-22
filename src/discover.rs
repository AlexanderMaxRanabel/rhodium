//discovery.rs
use std::fs::File;
use std::io::{BufRead, BufReader};
use reqwest;
use colored::*;

pub async fn discover(url: String, wordlist: String, depth: u8) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let file = File::open(wordlist).expect("Failed to open the file.");

    // Create a BufReader to read the file efficiently
    let reader = BufReader::new(file);
        // Iterate over each line in the wordlist
        for line in reader.lines() {
            if let Ok(word) = line {
                let target = url.clone().to_string() + &word.to_string();
                let response = reqwest::get(target.clone()).await?;
                let status = response.status().to_string();
                let code:Option<&str> = status.split_whitespace().nth(0);
                let result = match code {
                    Some(code) => code.to_string(),
                    None => String::from("Unknown"),
                };
                match depth {
                    0 => {
                        if response.status().is_success() {
                            println!("{} {} {}", result.clone().cyan(), word, target.clone().magenta());
                        } else {
                            println!("{} {} {}", result.clone().red(), word, target.clone().magenta());
                        }
                    },
                    1 => {
                        println!("{} {} {}", result.clone().red(), word, target.clone().magenta());
                        let mut indicator: i32 = -1;
                        //let resp = response.json::<HashMap<String, String>>().await?;
                        let code:Option<&str> = status.split_whitespace().nth(0);
                        let result = match code {
                            Some(code) => code.to_string(),
                            None => String::from("Unknown"),
                        };
                        println!("{} {} {}", result.clone().green(), word, target.clone().magenta());
                        let mut successful: Vec<String> = vec![];
                        if response.status().is_success() {
                            successful.push(target.clone());
                            indicator += 1;
                            let new_target = successful[indicator as usize].clone() + "/" + &word;
                            let new_response = reqwest::get(new_target.clone()).await?;
                            let new_status = new_response.status().to_string();
                            let new_code:Option<&str> = new_status.split_whitespace().nth(0);
                            let new_result = match new_code {
                                Some(new_code) => new_code.to_string(),
                                None => String::from("Unknown"),
                            };
                            println!("{} {} {} {}",depth.to_string().purple().italic() ,new_result.cyan(), word, new_target.magenta());
                        } else {
                            continue;
                        }
                    },
                    _ => {
                        println!("Unknown Depth");
                        std::process::exit(1)
                    }
                }
            }
        };
    Ok(())
} //INCOMPLETE. how can store how much lines are in the file in a var in this context