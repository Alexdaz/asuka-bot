#![allow(deprecated)]

mod commands;
mod system;
mod structs;

use std::env;
use dotenvy::dotenv;

use serenity::client::EventHandler;

use serenity::gateway::ActivityData;

use serenity::{async_trait, Client};
use serenity::prelude::Context;
use serenity::framework::standard::{
    macros::group,
    StandardFramework, Configuration
};
use std::sync::Arc;
use tokio::sync::Mutex;

use serenity::model::id::{MessageId, ChannelId};

use serenity::model::{
    channel::Reaction,
    gateway::Ready,
    gateway::GatewayIntents
};

use serenity::model::channel::ReactionType;

use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};

use system::console::{a_print, banner};

use crate::commands::poll::*;

use commands::help::HELP_COMMAND;

use commands::animedex::ANIMEDEX_COMMAND;

use commands::felizjueves::FELIZJUEVES_COMMAND;

use commands::yugi::YUGI_COMMAND;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
  async fn ready(&self, _: Context, ready: Ready) {
    banner(ready.user.name.to_string());
  }

  async fn reaction_add(&self, ctx: Context, add_reaction: Reaction) {
    perform_reaction! { (ctx, ReactionEvent::Reaction(&add_reaction)) |poll, number| {
      poll.answerers[number.unwrap()].push(add_reaction.user_id.unwrap());
    }}
  }

  async fn reaction_remove(&self, ctx: Context, removed_reaction: Reaction) {
    a_print("Single reaction remove".to_string());

    perform_reaction! { (ctx, ReactionEvent::Reaction(&removed_reaction)) |poll, number| {
      let ans = &mut poll.answerers[number.unwrap()];
      if let Some(i) = ans.iter().position(|x| x == &removed_reaction.user_id.unwrap()) {
        ans.remove(i);
      }
    }}
  }

  async fn reaction_remove_all(&self, ctx: Context, channel_id: ChannelId, removed_from_message_id: MessageId) {
    a_print("All reactions removed".to_string());

    perform_reaction! { (ctx, ReactionEvent::RemoveAll(channel_id, removed_from_message_id)) |poll, _| {
      for answers in poll.answerers.iter_mut() {
        answers.clear();
      }
    }}
  }
}

#[group]
#[commands(poll, animedex, felizjueves, help, yugi)]
struct General;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let action: String = env::var("ACTIVITY").expect("There is no activity.");
    let prefix: String = env::var("PREFIX").expect("There is no prefix.");
    let token:  String = env::var("DISCORD_TOKEN").expect("There is no token.");
    let debug:  String = env::var("DEBUG").expect("Debug option is missing.");

    if debug.eq("1")
    {
      let logfile: FileAppender = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d([%Y.%m.%d %H:%M:%S]:)} | {({l}):5.5} | {f}:{L} — {m}{n}\n")))
        .build("log/asuka_bot.log").unwrap();

      let config: Config = Config::builder()
          .appender(Appender::builder().build("logfile", Box::new(logfile)))
          .build(Root::builder()
                     .appender("logfile")
                     .build(LevelFilter::Info)).unwrap();

      log4rs::init_config(config).unwrap();
    }

    let framework: StandardFramework = StandardFramework::new().group(&GENERAL_GROUP);
    framework.configure(Configuration::new().prefix(prefix).case_insensitivity(true));

    let activity: ActivityData = ActivityData::custom(action);

    let intents: GatewayIntents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client: Client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .type_map_insert::<PollsKey>(Arc::new(Mutex::new(PollsMap::new())))
        .activity(activity)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        a_print(format!("ERROR: {:?}", why));
    }
}
