mod bot;
mod dispatcher;
mod telegram;

use crate::telegram::Telegram;
use std::error::Error;
use std::sync::Arc;
use teloxide::prelude::*;
use teloxide::types::Me;
use teloxide::utils::command::{BotCommand, ParseError};

#[derive(Debug)]
enum Command {
    Empty,
}

impl BotCommand for Command {
    fn descriptions() -> String {
        String::from("test")
    }

    fn parse<N>(s: &str, bot_name: N) -> Result<Self, ParseError>
    where
        N: Into<String>,
    {
        log::info!("parse command: {} {}", bot_name.into(), s);
        Ok(Command::Empty)
    }
}

async fn answer_command(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    log::info!("answer_command {:?}", command);

    cx.answer("xxx").await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    log::info!("Bot started");

    // let bot = Bot::from_env().auto_send();
    // let Me { user: bot_user, .. } = bot.get_me().await.unwrap();
    // let bot_name = bot_user.username.expect("Bots must have username");

    // println!("{:?}", &bot_user.full_name());

    // teloxide::commands_repl(bot, bot_name, answer_command).await;

    let bot = Bot::from_env().auto_send();
    // let tg = Telegram::from_id();

    bot::run(bot.clone()).await
}
