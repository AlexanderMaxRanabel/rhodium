//discover_filter.rs

use std::fs::File;
use std::io::{BufRead, BufReader};

use reqwest;
use colored::*;

pub async fn discover_filter(url: String, wordlist: String, filteree: String, type_fiter: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let file = File::open(wordlist).expect("Failed to open the file.");

    // Create a BufReader to read the file efficiently
    let reader = BufReader::new(file);

    // Iterate over each line in the wordlist
    for line in reader.lines() {
        if let Ok(word) = line {
            match type_fiter.as_str() {
                "--bycode" => {
                    let target = url.clone().to_string() + &*word.to_string();
                    let response = reqwest::get(target.clone()).await?;
                    let status = response.status().to_string();
                    //let resp = response.json::<HashMap<String, String>>().await?;

                    let code:Option<&str> = status.split_whitespace().nth(0);
                    let result = match code {
                        Some(code) => code.to_string(),
                        None => String::from("Unknown"),
                    };
                    match filteree.as_str() {
                        "200" => {
                            if response.status().is_success() {
                                println!("{} {} {}", result.green(), word, target.clone().magenta());
                            }
                        },
                        "404" | "401" | "402" | "403" => {
                            if response.status().is_client_error() {
                                println!("{} {} {}", result.green(), word, target.clone().magenta());
                            }
                        },

                        "301" | "307" | "302" => {
                            if response.status().is_redirection() {
                                println!("{} {} {}", result.green(), word, target.clone().magenta());
                            }
                        },

                        "500" | "502" => {
                            if response.status().is_server_error() {
                                println!("{} {} {}", result.green(), word, target.clone().magenta());
                            }
                        },
                        _ => println!("Unknown Status code")
                    }
                },

                "--bytype" => {
                    let target = url.clone().to_string() + &*word.to_string();
                    let response = reqwest::get(target.clone()).await?;
                    let status = response.status().to_string();
                    //let resp = response.json::<HashMap<String, String>>().await?;

                    let code:Option<&str> = status.split_whitespace().nth(0);
                    let result = match code {
                        Some(code) => code.to_string(),
                        None => String::from("Unknown"),
                    };
                    match filteree.as_str() {
                        "success" => {
                            if response.status().is_success() {
                                println!("{} {} {}", result.green(), word, target.clone().magenta());
                            }
                        },

                        "client_error" => {
                            if response.status().is_client_error() {
                                println!("{} {} {}", result.green(), word, target.clone().magenta());
                            }
                        },

                        "redirection" => {
                            if response.status().is_redirection() {
                                println!("{} {} {}", result.green(), word, target.clone().magenta());
                            }
                        },

                        "server_error" => {
                            if response.status().is_server_error() {
                                println!("{} {} {}", result.green(), word, target.clone().magenta());
                            }
                        },
                        _ => println!("Unknown Type")
                    }
                },
                _ => println!("Unknown Type")
            }
        }
    }
    Ok(())
}