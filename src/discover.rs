//discovery.rs
use std::fs::File;
use std::io::{BufRead, BufReader};
use colored::*;


pub async fn discover(url: String, wordlist: String, depth: u8) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let file = File::open(wordlist)?;
    let mut successful: Vec<String> = vec![];

    // Create a BufReader to read the file efficiently
    let reader = BufReader::new(file);
    // Iterate over each line in the wordlist
    for line in reader.lines() {
        if let Ok(word) = line {
            let target = format!("{}/{}", url, word);
            let response = reqwest::get(target.clone()).await?;
            let status = response.status().to_string();
            let code:Option<&str> = status.split_whitespace().next();
            let result = match code {
                Some(code) => code.to_string(),
                None => String::from("Unknown"),
            };

            match depth {
                0 => {
                    if response.status().is_success() {
                        println!("{} {} {}", &result.cyan(), word, target.clone().magenta());
                    } else {
                        println!("{} {} {}", &result.red(), word, target.clone().magenta());
                    }
                },

                1 => {
                    let code: Option<&str> = status.split_whitespace().next();
                    let result = match code {
                        Some(code) => code.to_string(),
                        None => String::from("Unknown"),
                    };

                    if response.status().is_success() {
                        println!("{} {} {}", &result.green(), word, target.clone().magenta());
                    } else {
                        println!("{} {} {}", &result.red(), word, target.clone().magenta());
                    }

                    let mut indicator: i32 = -1;  
                    
                    if response.status().is_success() {
                        successful.push(target.clone());
                        indicator += 1;
                        let new_target = successful[indicator as usize].clone() + "/" + &word;
                        let new_response = reqwest::get(new_target.clone()).await?;
                        let new_status = new_response.status().to_string();
                        let new_code:Option<&str> = new_status.split_whitespace().next();
                        let new_result = match new_code {
                            Some(new_code) => new_code.to_string(),
                            None => String::from("Unknown Code"),
                        };
                        println!("{} {} {} {}", depth.to_string().purple().italic() ,new_result.cyan(), word, new_target.magenta());
                    } else {
                        continue;
                    }
                },

                2 => {
                    let code:Option<&str> = status.split_whitespace().next();
                    let result = match code {
                        Some(code) => code.to_string(),
                        None => String::from("Unknown"),
                    };

                    if response.status().is_success() {
                        println!("{} {} {}", &result.green(), word, target.clone().magenta());
                    } else {
                        println!("{} {} {}", &result.red(), word, target.clone().magenta());
                    }
                    let mut indicator: i32 = -1; 

                    let mut successful: Vec<String> = vec![];
                    if response.status().is_success() {
                        successful.push(target.clone());
                        indicator += 1;
                        let new_target = successful[indicator as usize].clone() + "/" + &word;
                        let new_response = reqwest::get(new_target.clone()).await?;
                        let new_status = new_response.status().to_string();
                        let new_code: Option<&str> = new_status.split_whitespace().next();
                        let new_result = match new_code {
                            Some(new_code) => new_code.to_string(),
                            None => String::from("Unknown"),
                        };

                        let mut new_successful: Vec<String> = vec![];
                        let mut new_indicator = -1;

                        println!("{} {} {} {}", "1".purple().italic(), new_result.cyan(), word, new_target.magenta());
                        if new_response.status().is_success() {
                            new_indicator += 1;
                            new_successful.push(new_target.clone());
                            let newest_target = new_successful[new_indicator as usize].clone() + "/" + &word;
                            let newest_response = reqwest::get(newest_target.clone()).await?;
                            let newest_status = newest_response.status().to_string();
                            let newest_code: Option<&str> = newest_status.split_whitespace().next();
                            let newest_result = match newest_code {
                                Some(newest_code) => newest_code.to_string(),
                                None => String::from("Unknown"),
                            };
                            println!("{} {} {} {}", depth.to_string().purple().italic(), newest_result.cyan(), word, newest_target.magenta());
                        }
                    } else {
                        continue;
                    }
                },
                _ => {
                    println!("{}: Unknown Depth: {}", "Error".red(), depth.to_string().magenta().italic());
                    std::process::exit(1)
                }
            }
        }
    };
    Ok(())
}
