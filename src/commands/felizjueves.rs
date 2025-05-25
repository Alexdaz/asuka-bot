use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;
use serenity::prelude::{Context, Mentionable};
use serenity::builder::{CreateAttachment, CreateEmbed, CreateMessage};

use chrono::{DateTime, Datelike, Weekday, Utc};

use crate::system::settings::load_config;

#[command]
#[description = "Te deseo un feliz jueves."]
pub async fn felizjueves(ctx: &Context, msg: &Message) -> CommandResult 
{
    let config_data = load_config();

    let tz: String = config_data.settings.timezone;
    
    let date: DateTime<Utc> = Utc::now();

    let tz_use: chrono_tz::Tz = tz.parse().unwrap();

    if date.with_timezone(&tz_use).weekday() == Weekday::Thu
    {
        let embed: CreateEmbed = CreateEmbed::new()
        .title("¡Feliz jueves! uwu")
        .image("attachment://assets/jueves.gif");
        
        let builder: CreateMessage = CreateMessage::new()
        .content(msg.author.mention().to_string())
        .add_file(CreateAttachment::path("./assets/jueves.gif").await.unwrap())
        .embed(embed);
        
        msg.channel_id.send_message(&ctx.http, builder).await?;
    }
    else 
    {
        msg.reply(&ctx.http, "¡Baka! Hoy no es jueves 🤬").await?;
    }

    return Ok(());
}