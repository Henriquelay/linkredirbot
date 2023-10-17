//! This is a simple bot that replies to every message containing a link with
//! a better version of the link, either by having better privacy or by having better previews for Telegram

#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]
#![warn(unused_crate_dependencies)]
#![deny(missing_docs)]
#![deny(missing_doc_code_examples)]

use futures::future::try_join_all;
use teloxide::prelude::*;

mod link;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        let new_links = link::map_links(&msg);
        let handles = new_links.into_iter().map(|new_link| async {
            bot.send_message(msg.chat.id, new_link)
                .allow_sending_without_reply(false)
                .reply_to_message_id(msg.id)
                .await
        });
        try_join_all(handles).await?;
        Ok(())
    })
    .await;
}
