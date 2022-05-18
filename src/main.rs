use futures::stream::TryStreamExt;
use markov_bot::config::Config;
use markov_bot::markov::types::ChatMarkovChain;
use mongodb::{bson::doc, options::ClientOptions, Client};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let Config {
        // bot_token,
        mongodb_user,
        mongodb_password,
        mongodb_db,
        mongodb_url,
        ..
    } = markov_bot::config::load_config_from_env();
    pretty_env_logger::init_timed();

    let connection_string = format!(
        "mongodb://{}:{}@{}",
        mongodb_user, mongodb_password, mongodb_url
    );
    let client_options = ClientOptions::parse(connection_string).await?;
    let client = Client::with_options(client_options)?;
    let db = client.database(&mongodb_db);
    let collection = db.collection::<ChatMarkovChain>("chat_markov_chain");
    let filter = doc! {"chat_id": 1};
    let mut cursor = collection.find(filter, None).await?;

    while let Some(entry) = cursor.try_next().await? {
        println!("{:?}", entry);
    }

    Ok(())
}
