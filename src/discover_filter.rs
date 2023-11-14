//discover_filter.rs

use std::fs::File;
use std::io::{BufRead, BufReader};
use colored::*;

pub async fn discover_filter(url: String, wordlist: String, filteree: String, type_fiter: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let file = File::open(wordlist)?;

    // Create a BufReader to read the file efficiently
    let reader = BufReader::new(file);

    // Iterate over each line in the wordlist
    for line in reader.lines() {
        if let Ok(word) = line {
            match type_fiter.as_str() {
                "--bycode" => {
                    let target = url.clone().to_string() + "/" + &word.to_string();
                    let response = reqwest::get(target.clone()).await?;
                    let status = response.status().to_string();
                    //let resp = response.json::<HashMap<String, String>>().await?;

                    let code:Option<&str> = status.split_whitespace().next();
                    let result = match code {
                        Some(code) => code.to_string(),
                        None => String::from("Unknown"),
                    };
                    match filteree.as_str() {
                        "200" => {
                            if response.status().is_success() {
                                println!("{} {} {}", &result.green(), word, target.clone().magenta());
                            }
                        },

                        "404" | "401" | "402" | "403" => {
                            if response.status().is_client_error() {
                                match filteree.as_str() {
                                    "404" => {
                                        println!("{} {} {}", &result.green(), word, target.clone().magenta());
                                    },

                                    "401" => {
                                        println!("{} {} {}", &result.green(), word, target.clone().magenta());
                                    },

                                    "402" => {
                                        println!("{} {} {}", &result.green(), word, target.clone().magenta());
                                    },

                                    "403" => {
                                        println!("{} {} {}", &result.green(), word, target.clone().magenta());
                                    },

                                    _ => println!("{}: Unknown HTTP Response Code", "Error".red())
                                }
                            }
                        },

                        "301" | "307" | "302" => {
                            if response.status().is_redirection() {
                                match filteree.as_str() {
                                    "301" => {
                                        println!("{} {} {}", &result.green(), word, target.clone().magenta());
                                    },

                                    "302" => {
                                        println!("{} {} {}", &result.green(), word, target.clone().magenta());
                                    },

                                    "307" => {
                                        println!("{} {} {}", &result.green(), word, target.clone().magenta());
                                    },

                                    _ => println!("{}: Unknown HTTP Response Code", "Error".red())
                                }
                            }
                        },

                        "500" | "502" => {
                            if response.status().is_server_error() {
                                match filteree.as_str() {
                                    "500" => {
                                        println!("{} {} {}", &result.green(), word, target.clone().magenta());
                                    },

                                    "502" => {
                                        println!("{} {} {}", &result.green(), word, target.clone().magenta());
                                    },

                                    _ => println!("{} {} {}", result.green(), word, target.clone().magenta())
                                }
                            }
                        },
                        _ => println!("{}: Unknown HTTP Response Code", "Error".red())
                    }
                },

                "--bytype" => {
                    let target = url.clone().to_string() + "/" + &word.to_string();
                    let response = reqwest::get(target.clone()).await?;
                    let status = response.status().to_string();
                    //let resp = response.json::<HashMap<String, String>>().await?;

                    let code:Option<&str> = status.split_whitespace().next();
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
                        _ => println!("{}: Unknown HTTP Response Code", "Error".red())
                    }
                },
                _ => println!("Unknown Type")
            }
        }
    }
    Ok(())
}
