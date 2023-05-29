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

fn map_link(link: &str) -> Result<String, String> {
    if link.contains("twitter.com") {
        Ok(link.replace("twitter.com", "fxtwitter.com"))
    } else if link.contains("igshid=") {
        // remove everything past igshid
        link.split("igshid=")
            .next()
            .map(std::string::ToString::to_string)
            .ok_or(format!("Failed to remove igshid from {link}"))
    } else {
        Ok(link.to_string())
    }
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        let links = links_from_msg(&msg);
        if links.is_empty() {
            const REPLY_TEXT: &str = "No link detected";

            bot.send_message(msg.chat.id, REPLY_TEXT).await?;
        } else {
            dbg!(msg.from().map(|user| &user.username), msg.text());
            for link in links {
                let reply = map_link(link);
                let reply = match reply {
                    Ok(reply) => reply,
                    Err(e) => format!("[DEBUG] Failed to map link: {e}"),
                };
                bot.send_message(msg.chat.id, reply).await?;
            }
        }
        Ok(())
    })
    .await;
}