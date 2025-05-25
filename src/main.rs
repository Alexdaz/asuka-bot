#![allow(deprecated)]

mod commands;
mod system;
mod structs;

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

use system::security::{decrypt, encrypt_env_var, token_exists};
use system::console::{a_print, banner};
use system::settings::load_config;

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
    a_print("Single reaction remove");

    perform_reaction! { (ctx, ReactionEvent::Reaction(&removed_reaction)) |poll, number| {
      let ans = &mut poll.answerers[number.unwrap()];
      if let Some(i) = ans.iter().position(|x| x == &removed_reaction.user_id.unwrap()) {
        ans.remove(i);
      }
    }}
  }

  async fn reaction_remove_all(&self, ctx: Context, channel_id: ChannelId, removed_from_message_id: MessageId) {
    a_print("All reactions removed");

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

    let config_data: system::settings::Data = load_config();

    if config_data.settings.debug == 1
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
    
    if token_exists() 
    {
      println!("Please enter your Discord token:");

      let mut token: String = String::new();
      
      if let Err(e) = std::io::stdin().read_line(&mut token) 
      {
          let msg: String = format!("Error reading input: {}", e);

          a_print(&msg);
          return;
      }

      let token: &str = token.trim();

      if token.is_empty() 
      {
          a_print("No token");
      } 
      else 
      {
          if let Err(e) = clearscreen::clear() 
          {
              let msg: String = format!("Failed to clear screen: {}", e);

              a_print(&msg);
          }

          encrypt_env_var(token);
      }
    }

    let framework: StandardFramework = StandardFramework::new().group(&GENERAL_GROUP);
    framework.configure(Configuration::new().prefix(config_data.settings.prefix).case_insensitivity(true));

    let activity: ActivityData = ActivityData::custom(config_data.settings.activity);

    let intents: GatewayIntents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let decrypted_token: String = decrypt();

    let mut client: Client = Client::builder(decrypted_token, intents)
        .event_handler(Handler)
        .framework(framework)
        .type_map_insert::<PollsKey>(Arc::new(Mutex::new(PollsMap::new())))
        .activity(activity)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await 
    {
        let msg: String = format!("ERROR: {:?}", why);

        a_print(&msg);
    }
}
