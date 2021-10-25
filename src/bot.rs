use log::*;
use std::error::Error;
use teloxide::prelude::*;
use teloxide::types::{Me, MessageLeftChatMember};
use teloxide::utils::command::BotCommand;
use teloxide::Bot;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio_stream::Stream;

#[derive(BotCommand, Debug)]
enum Command {
    Help,
}

async fn handle_commands(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    cmd: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    info!("cx: {:?}", cx.update);
    info!("cx: {:?}", cmd);

    cx.answer("response").await?;
    Ok(())
}

/// New messages
async fn handle_messages(rx: DispatcherHandlerRx<AutoSend<Bot>, Message>) {
    UnboundedReceiverStream::new(rx)
        // .for_each_concurrent()
        // .commands::<Command, _>("sbwtw-bot")
        .for_each_concurrent(None, |cx| async move {
            // handle_commands(cx, cmd).await.log_on_error().await;
            info!("handle_messages: {:?}", cx.update);
            handle_commands(cx, Command::Help)
                .await
                .log_on_error()
                .await
        })
        .await;
}

/// New group member messages
async fn handle_chat_member_updated(rx: DispatcherHandlerRx<AutoSend<Bot>, ChatMemberUpdated>) {
    UnboundedReceiverStream::new(rx)
        .for_each_concurrent(None, |m| async move {
            info!("handle_chat_member_updated: {:?}", m.update);
        })
        .await;
}

async fn handle_my_chat_members(rx: DispatcherHandlerRx<AutoSend<Bot>, ChatMemberUpdated>) {
    UnboundedReceiverStream::new(rx)
        .for_each_concurrent(None, |m| async move {
            info!("handle_my_chat_members: {:?}", m.update);
        })
        .await;
}

pub async fn run(bot: AutoSend<Bot>) {
    let Me { user, .. } = bot.get_me().await.expect("Couldn't get myself!");
    let name = user.username.expect("Bots *must*  have username");
    log::info!("Start Bot {}", name);

    // message dispatcher
    let mut dispatcher = Dispatcher::new(bot)
        .messages_handler(handle_messages)
        .chat_members_handler(handle_chat_member_updated)
        .my_chat_members_handler(handle_my_chat_members)
        .setup_ctrlc_handler();

    dispatcher.dispatch().await;
}
