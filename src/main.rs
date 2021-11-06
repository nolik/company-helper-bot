use std::env;
use std::error::Error;

use teloxide::{prelude::*, utils::command::BotCommand};

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting bot...");

    // TODO: move to lazy lazy_static
    let company_info = env::var("COMPANY_INFO").expect("COMPANY_INFO must be set");
    let discount_codes = env::var("DISCOUNT_CODES").expect("DISCOUNT_CODES must be set");

    let bot = Bot::from_env().auto_send();
    teloxide::commands_repl(bot, "godel-helper", answer).await;
}

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "Supported commands:")]
enum Command {
    #[command(description = "Show list of commands")]
    Help,
    #[command(description = "Show official company name")]
    Info,
    #[command(description = "Show OZ discount codes")]
    Discount,
}

async fn answer(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {

    match command {
        Command::Help => cx.answer(Command::descriptions()).await?,
        Command::Info => cx.answer(format!("{}", "company_info")).await?,
        Command::Discount => cx.answer(format!("OZ codes:{}", "discount_codes")).await?,
    };

    Ok(())
}
