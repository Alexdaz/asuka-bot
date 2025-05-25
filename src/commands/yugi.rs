use std::ops::Index;
use std::time::Duration;

use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::builder::{CreateEmbed, CreateMessage};
use serenity::model::prelude::Message;
use serenity::prelude::Context;

use reqwest::{Client, StatusCode};

use crate::structs::yugi_json::{Daum, Root};

#[command]
pub async fn yugi(ctx: &Context, msg: &Message, args: Args) -> CommandResult 
{
    if args.message().is_empty()
    {
        msg.reply(&ctx.http, "Baka! Write something 🤬").await?;
        return Ok(());
    }

    let name: String = args.message().to_lowercase();

    let params: [(&str, &String); 1] = [
        ("name", &name)
    ];

    let response: reqwest::Response = Client::new()
    .get(reqwest::Url::parse_with_params("https://db.ygoprodeck.com/api/v7/cardinfo.php", &params)?)
    .timeout(Duration::from_secs(60 * 30))
    .send()
    .await?;

    if response.status() == StatusCode::BAD_REQUEST 
    {
        msg.reply(&ctx.http, "Baka! That card does not exist 🤬").await?;
        return Ok(());
    }

    let json: Root = response.json().await?;
    
    let data_card: &Daum = json.data.index(0);

    let embed: CreateEmbed = CreateEmbed::new()
        .color(0x371F76)
        .title("Yugi card info 🎴")
        .description("")
        .fields(vec![
            ("Name"       , data_card.name.to_string(), false),
            ("Type"       , data_card.type_field.to_string(), false),
            ("Attack"     , data_card.atk.unwrap_or(0).to_string(), false),
            ("Defense"    , data_card.def.unwrap_or(0).to_string(), false),
            ("Level"      , data_card.level.unwrap_or(0).to_string(), false),
            ("Race"       , data_card.race.to_string(), false),
            ("Attribute"  , data_card.attribute.clone().unwrap_or("-".to_string()), false),
            ("Description", data_card.desc.to_string(), false),
        ])
        .image(data_card.card_images.index(0).image_url.to_string());

    let builder: CreateMessage = CreateMessage::new().embed(embed);

    msg.channel_id.send_message(&ctx.http, builder).await?;

    return Ok(());
}