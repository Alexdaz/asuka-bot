use std::ops::Index;

use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::builder::{CreateEmbed, CreateMessage};
use serenity::model::prelude::Message;
use serenity::prelude::Context;

use reqwest::Client;

use crate::structs::mal_json::{Daum, Root};

#[command]
pub async fn animedex(ctx: &Context, msg: &Message, args: Args) -> CommandResult 
{
    if args.message().is_empty()
    {
        msg.reply(&ctx.http, "Baka! Write something 🤬").await?;
        return Ok(());
    }

    let name: String = args.message().to_lowercase();

    let params: [(&str, &str); 2] = [
        ("limit", "1"),
        ("q", &name)
    ];

    let json: Root = Client::new()
        .get(reqwest::Url::parse_with_params("https://api.jikan.moe/v4/anime", &params)?)
        .send()
        .await?
        .json()
        .await?;

    if json.data.len() == 0
    {
        msg.reply(&ctx.http, "Baka! That anime does not exist 🤬").await?;
        return Ok(());
    }

    let data_anime: &Daum = json.data.index(0);

    let is_trash: &str = if data_anime.score.unwrap_or(0.0) < 7.50 { "True ✅" } else { "False 🚫" };

    let embed: CreateEmbed = CreateEmbed::new()
        .color(0xD53C55)
        .title("Anime Info ⛩️")
        .description("")
        .fields(vec![
            ("Title"     , data_anime.title.to_string()         , false),
            ("Episodes"  , data_anime.episodes.to_string()      , false),
            ("Stream"    , data_anime.type_field.to_string()    , false),
            ("Score"     , data_anime.score.unwrap_or(0.0).to_string(), false),
            ("Is trash?" , is_trash.to_string()                 , false),
        ])
        .image(data_anime.images.webp.image_url.to_string());

    let builder: CreateMessage = CreateMessage::new().embed(embed);

    msg.channel_id.send_message(&ctx.http, builder).await?;

    return Ok(());
}
