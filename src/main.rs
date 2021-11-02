use std::env;

use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting bot...");

    let bot = Bot::from_env().auto_send();
    teloxide::repl(bot, move |message| {
        async move {
            respond(())
        }
    })
        .await;
}

