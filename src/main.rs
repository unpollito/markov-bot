use simple_telegram_bot::SimpleTelegramBot;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = markov_bot::config::load_config_from_env();
    pretty_env_logger::init_timed();
    let db = markov_bot::db::Db::new(&config).await?;

    let bot = SimpleTelegramBot::new(config.bot_token.clone());
    let sender = bot.sender();
    let mut stream = bot.updater().updates();

    log::info!("Starting bot...");

    loop {
        match stream.recv().await {
            None => break,
            Some(response) => {
                for update in response.result {
                    if let Some(message) = update.message {
                        let text = match message.text {
                            Some(text) => text,
                            None => String::from(""),
                        };
                        let chat_id = message.chat.id;
                        let username = message.from.username;

                        log::debug!(
                            "Received message from {} (chat ID: {}): \"{}\"",
                            username,
                            chat_id,
                            text
                        );

                        if !message.from.is_bot {
                            let mut chain = db.get_by_chat_id(chat_id).await?;
                            if text == "/markov"
                                || text.starts_with("/markov ")
                                || text.starts_with("/markov@")
                            {
                                let message = match chain.generate_sentence() {
                                    Some(sentence) => sentence,
                                    None => {
                                        "Markov says: sorry, I don't know what to say".to_string()
                                    }
                                };
                                sender.send_message(chat_id, &message).await;
                            } else if text == "/markov_clear"
                                || text.starts_with("/markov_clear ")
                                || text.starts_with("/markov_clear@")
                            {
                                db.delete_chain(chat_id).await?;
                            } else if !text.is_empty() && !text.starts_with("/") {
                                for line in text.lines() {
                                    chain.add_sentence(line);
                                }
                                db.save_chain(&chain).await?;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
