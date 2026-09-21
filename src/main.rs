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
use teloxide::{
    prelude::*,
    types::{LinkPreviewOptions, ReplyParameters},
};

mod link;

#[tokio::main]
async fn main() {
    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        let new_links = link::map_links(&msg);
        let handles = new_links.into_iter().map(|new_link| async {
            bot.send_message(msg.chat.id, &new_link)
                .link_preview_options(LinkPreviewOptions {
                    is_disabled: false,
                    url: Some(new_link),
                    prefer_small_media: false,
                    prefer_large_media: true,
                    show_above_text: true,
                })
                .reply_parameters(ReplyParameters::new(msg.id))
                .await
        });
        try_join_all(handles).await?;
        Ok(())
    })
    .await;
}
