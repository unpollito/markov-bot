use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, to_document},
    options::ClientOptions,
    Client, Collection,
};
use std::{collections::HashMap, error::Error, fmt};

use crate::{
    config::Config,
    markov::{constants::*, types::*},
};

pub struct Db {
    collection: Collection<ChatMarkovChain>,
}

impl Db {
    pub async fn new(config: Config) -> Result<Db, mongodb::error::Error> {
        let Config {
            mongodb_user,
            mongodb_password,
            mongodb_db,
            mongodb_url,
            ..
        } = config;
        let connection_string = format!(
            "mongodb://{}:{}@{}",
            mongodb_user, mongodb_password, mongodb_url
        );
        let client_options = ClientOptions::parse(connection_string).await?;
        let client = Client::with_options(client_options)?;
        let db = client.database(&mongodb_db);
        let collection = db.collection::<ChatMarkovChain>("chat_markov_chain");
        return Ok(Db { collection });
    }

    pub async fn get_chat_id(&self, chat_id: i64) -> Result<ChatMarkovChain, Box<dyn Error>> {
        let filter = doc! {"chat_id": chat_id};
        let mut cursor = self.collection.find(filter, None).await?;
        let mut chat_data: Option<ChatMarkovChain> = None;

        while let Some(entry) = cursor.try_next().await? {
            if chat_data.is_some() {
                return Err(Box::new(DbError::new(format!(
                    "Multiple entries for the same chat_id {}",
                    chat_id
                ))));
            }
            chat_data = Some(entry);
        }

        match chat_data {
            None => Ok(Db::get_empty_chain(chat_id)),
            Some(entry) => {
                Db::validate_chain(&entry)?;
                Ok(entry)
            }
        }
    }

    pub async fn save_chain(&self, chain: &ChatMarkovChain) -> Result<(), Box<dyn Error>> {
        Db::validate_chain(chain)?;
        let entries_document = to_document(&chain);
        if let Err(e) = entries_document {
            return Err(Box::new(e));
        }
        self.collection
            .update_one(
                doc! {"chat_id": chain.chat_id},
                doc! {"$set": entries_document.unwrap() },
                None,
            )
            .await?;
        Ok(())
    }

    fn get_empty_chain(chat_id: i64) -> ChatMarkovChain {
        let mut entries: HashMap<String, Vec<ChatMarkovChainSuccessor>> = HashMap::new();
        entries.insert(String::from(MARKOV_CHAIN_START), vec![]);
        entries.insert(String::from(MARKOV_CHAIN_END), vec![]);
        return ChatMarkovChain { chat_id, entries };
    }

    fn validate_chain(chain: &ChatMarkovChain) -> Result<(), DbError> {
        if chain.entries.get(MARKOV_CHAIN_START).is_none() {
            return Err(DbError::new(format!(
                "No chain start for chat ID {}",
                chain.chat_id
            )));
        }
        if chain.entries.get(MARKOV_CHAIN_END).is_none() {
            return Err(DbError::new(format!(
                "No chain end for chat ID {}",
                chain.chat_id
            )));
        }
        let entries = &chain.entries;
        for (entry_word, entry) in entries.into_iter() {
            for successor in entry {
                if chain.entries.get(&successor.word).is_none() {
                    return Err(DbError::new(format!(
                        "Could not find successor {} for word {} in chat ID {}",
                        &successor.word, &entry_word, chain.chat_id
                    )));
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct DbError {
    message: String,
}

impl DbError {
    pub fn new(message: String) -> DbError {
        DbError { message }
    }
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for DbError {}
