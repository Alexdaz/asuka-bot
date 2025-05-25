use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;
use serenity::prelude::Context;
use serenity::builder::{CreateEmbed, CreateMessage};

use crate::system::settings::load_config;

#[command]
pub async fn help(ctx: &Context, msg: &Message) -> CommandResult
{
    let config_data = load_config();

    let prefix: char = config_data.settings.prefix;
    
    let embed: CreateEmbed = CreateEmbed::new()
        .color(0xE67E22)
        .title("Heyo!!! Here's the help 🛟")
        .description(format!("My prefix is: {}", prefix))
        .fields(vec![
            ("animedex", "Enter the anime you are interested in, and I will provide you with the most relevant data. 🌸".to_string(), false),
            ("yugi", "Enter the name of the card and I will give you the most relevant data. 🎴".to_string(), false),
            ("felizjueves", "I wish you a happy Thursday (obviously it only works on Thursdays 😒).".to_string(), false),
            ("poll", "I can create a poll for you, placing first the Title in quotation marks and then the options, also in quotation marks. 📊

                      Example: \"What we are going to eat today?\" \"Pizza\" \"Tacos\" \"Sushi\" 
                      The number of options is the whole alphabet.".to_string(), false),
            ("help", "Show this message. 😺".to_string(), false),
            ("", "I'm here for you! :3".to_string(), false),
            ("", "**I hope you have a wonderful day! uwu**".to_string(), false),
        ]);

    let builder: CreateMessage = CreateMessage::new()
        .embed(embed);

    msg.channel_id.send_message(&ctx.http, builder).await?;

    return Ok(());
}
