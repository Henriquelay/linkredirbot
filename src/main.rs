//! This is a simple bot that replies to every message containing a link with
//! a better version of the link, either by having better privacy or by having better previews for Telegram

#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]
#![warn(unused_crate_dependencies)]
#![deny(missing_docs)]
#![deny(missing_doc_code_examples)]

use teloxide::{prelude::*, types::MessageEntityKind};

fn links_from_msg(msg: &Message) -> Vec<&str> {
    let entities = msg.parse_entities().unwrap_or_default();
    entities
        .iter()
        .filter_map(|entity| match entity.kind() {
            MessageEntityKind::Url => Some(entity.text()),
            MessageEntityKind::TextLink { url, .. } => Some(url.as_str()),
            _ => None,
        })
        .collect()
}

fn map_link(link: &str) -> Option<String> {
    match link {
        _ if link.contains("twitter.com") && !link.contains("fxtwitter") => {
            Some(link.replace("twitter.com", "fxtwitter.com"))
        }
        _ if link.contains("x.com") && !link.contains("fixupx") => {
            Some(link.replace("x.com", "fixupx.com"))
        }
        _ if link.contains("tiktok.com") && !link.contains("vxtiktok") => {
            Some(link.replace("tiktok.com", "vxtiktok.com"))
        }
        _ if link.contains("instagram.com") && !link.contains("ddinstagram") => {
            let output_link = link.replace("instagram.com", "ddinstagram.com");
            output_link.split("igshid=").next().map(str::to_string)
        }

        _ => None,
    }
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        let links = links_from_msg(&msg);
        if !links.is_empty() {
            dbg!(msg.from().map(|user| &user.username), msg.text());
            let new_links = links.iter().filter_map(|link| map_link(link));
            for new_link in new_links {
                bot.send_message(msg.chat.id, new_link)
                    .allow_sending_without_reply(false)
                    .reply_to_message_id(msg.id)
                    .await?;
            }
        }
        Ok(())
    })
    .await;
}
