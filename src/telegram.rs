use std::sync::Arc;
use teloxide::prelude::*;
use teloxide::types::ChatId;
use teloxide::{Bot, RequestError};

pub struct Telegram {
    chat_id: ChatId,
    bot: Arc<Bot>,
}

impl Telegram {
    pub fn new(chat_id: ChatId, bot: Arc<Bot>) -> Self {
        Self { bot, chat_id }
    }

    pub async fn send<T: Into<String>>(&self, text: T) -> Result<(), RequestError> {
        self.bot
            .send_message(self.chat_id.clone(), text)
            .send()
            .await?;

        Ok(())
    }
}
