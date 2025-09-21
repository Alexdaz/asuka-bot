use std::env;

use colored::Colorize;
use colored::ColoredString;

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
const BANNER_WIDTH: usize = 80;

const PINK_COLOR: (u8, u8, u8) = (255, 144, 234);

fn accent(text: &str) -> ColoredString 
{
    let (r, g, b) = PINK_COLOR;

    return text.truecolor(r, g, b);
}

pub fn banner(username: &str)
{
    println!("{}", accent(TITLE));
    println!("{}Username: {}", TABS, accent(username));
    println!("{}Version: {}", TABS, accent(VERSION));
    println!(
        "{}OS: {}",
        TABS,
        accent(&format!(
            "{}{}",
            env::consts::OS[..1].to_uppercase(),
            &env::consts::OS[1..]
        ))
    );
    println!("{}Developed by: {}", TABS, accent(AUTHORS));
    println!("{}", accent(&("\n".to_owned() + &"=".repeat(BANNER_WIDTH))));
    
    a_print("It's alive!");
}


pub fn a_print(text: &str)
{
    let config_data = load_config();

    if config_data.settings.debug == 1
    {
        let date: String = Utc::now().format("[%Y.%m.%d %H:%M:%S]:").to_string();

        println!("{} {}", accent(&date), accent(text));
    }
}
