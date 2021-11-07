#[macro_use]
extern crate lazy_static;

use std::env;
use std::error::Error;

use teloxide::{prelude::*, utils::command::BotCommand};

lazy_static! {
    static ref COMPANY_INFO: String = env::var("COMPANY_INFO").expect("COMPANY_INFO must be set");
    static ref DISCOUNT_CODES: String =
        env::var("DISCOUNT_CODES").expect("DISCOUNT_CODES must be set");
}

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting bot...");

    let bot = Bot::from_env().auto_send();
    teloxide::commands_repl(bot, "company-helper", answer).await;
}

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "Supported commands:")]
enum Command {
    #[command(description = "Show list of commands")]
    Help,
    #[command(description = "Show official company name")]
    Info,
    #[command(description = "Show discount codes")]
    Discount,
}

async fn answer(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Help => cx.answer(Command::descriptions()).await?,
        Command::Info => cx.answer(format!("{}", COMPANY_INFO.to_owned())).await?,
        Command::Discount => cx.answer(format!("OZ codes:{}", DISCOUNT_CODES.to_owned())).await?,
    };

    Ok(())
}
