mod discover;
mod discover_filter;

use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    //monologue0 --url1 https://discord.com/2 --wordlist3 test.txt4  --filter5 --bycode6 4047
    if args.len() > 5 {
        let argument = &args[1].to_string();
        match argument.as_str() {
            "--url" => {
                let url = &args[2].to_string();
                let wordlist = &args[4].to_string();
                let mode = &args[5].to_string();
                match mode.as_str() {
                    "--normal" => {
                        let _ = tokio::task::spawn(discover::discover(url.clone(), wordlist.clone())).await;
                    },
                    "--filter" => {
                        let filteree = &args[7].to_string();
                        let type_filter = &args[6].to_string();
                        let _ = tokio::task::spawn(discover_filter::discover_filter(url.clone(),wordlist.clone(), filteree.clone(), type_filter.clone())).await;
                    },
                    _ => println!("Unknown mode")
                }
            },

            "help" => {
                println!("--help for help");
                println!("usage: --url [website] --wordlist [wordlist] [optional]");
            },
            _  => println!("Unknown Argument. Please --help"),
        }
    } else {
        println!("Insufficient Arguments");
    }
}
