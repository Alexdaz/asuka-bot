use std::env;

use colored::Colorize;

use chrono::Utc;

use super::settings::load_config;

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
    println!("{}OS: {}", TABS, format!("{}{}", env::consts::OS.chars().next().unwrap().to_uppercase(), 
    &env::consts::OS[1..]).truecolor(255, 144, 234));
    
    println!("{}Developed by: {}", TABS, AUTHORS.truecolor(255, 144, 234));
    println!("{}", "\n================================================================================"
    .truecolor(255, 144, 234));
    a_print("It's alive!");
}


pub fn a_print(text: &str)
{
    let config_data = load_config();

    if config_data.settings.debug == 1
    {
        let date: String = Utc::now().format("[%Y.%m.%d %H:%M:%S]:").to_string();
        println!("{} {}", date.truecolor(255, 144, 234), text.truecolor(255, 144, 234));
    }
}
