use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::prelude::{Context, TypeMapKey, TypeMap, Mentionable};

use serenity::model::channel::ReactionType;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use serenity::model::id::{ChannelId, MessageId, UserId};

use serenity::model::channel::{Message, Reaction};

pub enum ReactionEvent<'a> {
    Reaction(&'a Reaction),
    RemoveAll(ChannelId, MessageId),
}

#[macro_export]
macro_rules! perform_reaction {
    (($ctx:expr, $reaction_event:expr) $body:expr) => {
      use ReactionEvent::{Reaction, RemoveAll};
      // Discard if it's our own reaction.
      if let Reaction(r) = $reaction_event {
        if r.user_id == Some($ctx.cache.current_user().id) {
          a_print("Reaction added by self, ignoring");
          return;
        }
      }

      let key = match $reaction_event {
        Reaction(r) => (r.channel_id, r.message_id),
        RemoveAll(c, m) => (c, m),
      };

      // Try to get poll for the given message otherwise return
      {
        let poll_data = $ctx.data.read().await;
        let poll_map = poll_data
          .get::<PollsKey>()
          .expect("Failed to retrieve polls map!")
          .lock()
          .await;
        if !poll_map.contains_key(&key) {
          a_print("Message not in polls map, ignoring");
          return;
        }
      }

      // reretrieve the map as writable
      let mut poll_data = $ctx.data.write().await;
      let mut poll_map = poll_data
        .get_mut::<PollsKey>()
        .expect("Failed to retrieve polls map!")
        .lock()
        .await;
      let poll = match poll_map.get_mut(&key) {
        None => {
          let msg: String = format!("Failed to get poll for {:?}", key);
          
          a_print(&msg);
          return;
        }
        Some(poll) => poll,
      };

      // nudges Rust towards the right type :)
      fn get_f<F: FnOnce(&mut Poll, Option<usize>)>(f: F) -> F {
        return f;
      }
      let f = get_f($body);

      match $reaction_event {
        Reaction(r) => match r.emoji {
          ReactionType::Unicode(ref s) => {
            let c = s.chars().nth(0).unwrap();
            let end_char = std::char::from_u32('🇦' as u32 + poll.answers.len() as u32 - 1)
              .expect("Failed to format emoji");
            if c < '🇦' || c > end_char {
              return;
            }
            let number = (c as u32 - '🇦' as u32) as usize;

            f(poll, Some(number));
          }
          _ => {
            a_print("Unknown emoji in reaction, ignoring");
            return;
          }
        },
        RemoveAll(..) => f(poll, None),
      }

      let content = render_message(&poll);

      key.0
        .edit_message(&$ctx.http, key.1, serenity::builder::EditMessage::new().content(&content) )
        .await
        .expect("Failed to edit message");

        a_print("Rerendered message");
    };
}

pub fn render_message(poll: &Poll) -> String {
    let mut message_text: String = format!("🗳️ **Poll:** {}\n\n", poll.question);
    let total_answerers: usize = poll.answerers.iter().map(|x: &Vec<UserId>| x.len()).sum::<usize>();

    for (i, (answer, users)) in poll.answers.iter().zip(poll.answerers.iter()).enumerate() {
        let emoji = std::char::from_u32('🇦' as u32 + i as u32).expect("Failed to format emoji");
        message_text.push(emoji);
        if total_answerers > 0 {
            let percent: f64 = users.len() as f64 / total_answerers as f64 * 100.;
            message_text.push_str(&format!(" {:.0}%", percent));
        }
        message_text.push(' ');
        message_text.push_str(answer);
        message_text.push_str(" (");
        for user in users {
            message_text.push_str(&user.mention().to_string());
            message_text.push_str(", ");
        }
        message_text.push_str(&format!("{} votos)", users.len()));
        message_text.push('\n');
    }

    return message_text;
}

pub struct PollsKey;

impl TypeMapKey for PollsKey {
    type Value = Arc<Mutex<PollsMap>>;
}

pub type PollsMap = HashMap<(ChannelId, MessageId), Poll>;

pub struct Poll {
    pub question: String,
    pub answers: Vec<String>,
    pub answerers: Vec<Vec<UserId>>,
}

#[command]
pub async fn poll(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let question: String = args.single_quoted::<String>()?;
    let answers: Vec<String> = args
        .quoted()
        .iter::<String>()
        .filter_map(
            |x: Result<String, serenity::all::standard::ArgError<std::convert::Infallible>>| x.ok(),
        )
        .collect::<Vec<_>>();

    let answers_len = answers.len();
    let poll: Poll = Poll {
        question: question,
        answerers: vec![Vec::new(); answers_len],
        answers: answers,
    };

    let message_text: String = render_message(&poll);
    let emojis: Vec<char> = (0..answers_len)
        .map(|i: usize| std::char::from_u32('🇦' as u32 + i as u32).expect("Failed to format emoji"))
        .collect::<Vec<_>>();

    let poll_msg: Message = msg.channel_id.say(&ctx.http, &message_text).await?;

    for &emoji in &emojis {
        poll_msg
            .react(&ctx.http, ReactionType::Unicode(emoji.to_string()))
            .await?;
    }

    let mut poll_data: tokio::sync::RwLockWriteGuard<'_, TypeMap> = ctx.data.write().await;

    let poll_map: &mut Arc<Mutex<HashMap<(ChannelId, MessageId), Poll>>> = poll_data
        .get_mut::<PollsKey>()
        .expect("Failed to retrieve polls map!");

    poll_map
        .lock()
        .await
        .insert((msg.channel_id, poll_msg.id), poll);

    return Ok(());
}
