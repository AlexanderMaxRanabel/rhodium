//discovery.rs
use std::fs::File;
use std::io::{BufRead, BufReader};
use reqwest;
use colored::*;

pub async fn discover(url: String, wordlist: String, depth: u8) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let file = File::open(wordlist).expect("Failed to open the file.");
    let reader = BufReader::new(file);

    let mut targets: Vec<String> = vec![url];

    for line in reader.lines() {
        if let Ok(word) = line {
            let mut successful: Vec<String> = Vec::new();

            for target in &targets {
                let new_target = format!("{}/{}", target, word);
                let response = reqwest::get(&new_target).await?;
                let status = response.status().to_string();
                let code = status.split_whitespace().next().unwrap_or("Unknown");
                let result = code.to_string();

                match depth {
                    0 => println!("{} {} {}", result.cyan(), word, new_target.magenta()),
                    1 => {
                        if response.status().is_success() {
                            println!("{} {} {}", result.green(), word, new_target.magenta());
                            successful.push(new_target.clone());
                        } else {
                            println!("{} {} {}", result.red(), word, new_target.magenta());
                        }
                    },
                    2 => {
                        if response.status().is_success() {
                            println!("{} {} {}", result.green(), word, new_target.magenta());
                            successful.push(new_target.clone());
                        } else {
                            println!("{} {} {}", result.red(), word, new_target.magenta());
                        }
                    },
                    _ => {
                        println!("Unknown Depth");
                        std::process::exit(1)
                    }
                }
            }

            if depth >= 1 {
                targets = successful;
            }
        }
    }

    Ok(())
}
