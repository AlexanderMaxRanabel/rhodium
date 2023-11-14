mod discover;
mod discover_filter;

use std::env;

use colored::*;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    //monologue0 --url1 https://discord.com/2 --wordlist3 test.txt4  --depth 2 --normal
    if args.len() > 6 {
        let log = r#"
      _               _ _                 
     | |             | (_)                
 _ __| |__   ___   __| |_ _   _ _ __ ___  
| '__| '_ \ / _ \ / _` | | | | | '_ ` _ \ 
| |  | | | | (_) | (_| | | |_| | | | | | |
|_|  |_| |_|\___/ \__,_|_|\__,_|_| |_| |_|
        "#;
        println!("{}", log.red());
        let argument = &args[1].to_string();
        match argument.as_str() {
            "--url" | "-u" => {
                let url = &args[2].to_string();
                let wordlist = &args[4].to_string();
                let mode = &args[7].to_string();
                let depth = &args[6].to_string();
                let depth:u8 = depth.parse().expect("Error while conversition to u8");
                if depth > 3 {
                    println!("{}: The Limit is 3", "Error".red());
                    std::process::exit(1);
                }
                match mode.as_str() {
                    "--normal" | "-n" => {
                        let _ = tokio::task::spawn(discover::discover(url.clone(), wordlist.clone(), depth)).await;
                    },

                    "--filter" | "-f" => {
                        //monologue0 --url1 https://discord.com/2 --wordlist3 test.txt4  --depth5 26 --filter7 --bycode8 4049
                        let type_filter = &args[8].to_string();
                        let filteree = &args[9].to_string();
                        let _ = tokio::task::spawn(discover_filter::discover_filter(url.clone(), wordlist.clone(), filteree.clone(), type_filter.clone())).await;
                    },

                    _ => println!("{}: Unknown Mode: {}", "Error".red(), mode),
                }
            },

            "--help" | "-h" => {
                println!("--help for help");
                println!("usage: --url [website] --wordlist [wordlist] [optional]");
            },
            _  => println!("Unknown Argument. Please --help"),
        }
    } else {
        println!("Insufficient Arguments");
    }
}
