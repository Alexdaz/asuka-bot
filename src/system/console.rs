use std::env;
use dotenvy::dotenv;

use colored::Colorize;

use chrono::Utc;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

const TITLE: &str = r#"
              █████╗ ███████╗██╗   ██╗██╗  ██╗ █████╗       ██╗██████╗ 
             ██╔══██╗██╔════╝██║   ██║██║ ██╔╝██╔══██╗     ██╔╝╚════██╗
             ███████║███████╗██║   ██║█████╔╝ ███████║    ██╔╝  █████╔╝
             ██╔══██║╚════██║██║   ██║██╔═██╗ ██╔══██║    ╚██╗  ╚═══██╗
             ██║  ██║███████║╚██████╔╝██║  ██╗██║  ██║     ╚██╗██████╔╝
             ╚═╝  ╚═╝╚══════╝ ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝      ╚═╝╚═════╝ 
"#;

const TABS: &str = "\t\t\t";

pub fn banner(username: String)
{
    println!("{}", TITLE.truecolor(255, 144, 234));
    println!("{}Username: {}", TABS, username.truecolor(255, 144, 234));
    println!("{}Version: {}", TABS, VERSION.truecolor(255, 144, 234));
    println!("{}OS: {}", TABS, env::consts::OS.truecolor(255, 144, 234));
    println!("{}Developed by: {}", TABS, AUTHORS.truecolor(255, 144, 234));
    println!("{}", "\n================================================================================"
    .truecolor(255, 144, 234));
    a_print("It's alive!".to_string());
}


pub fn a_print(text: String)
{
    dotenv().ok();
    let debug: String = env::var("DEBUG").expect("Debug option is missing.");

    if debug.eq("1")
    {
        let date: String = Utc::now().format("[%Y.%m.%d %H:%M:%S]:").to_string();
        println!("{} {}", date.truecolor(255, 144, 234), text.truecolor(255, 144, 234));
    }
}
