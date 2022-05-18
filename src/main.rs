use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = markov_bot::config::load_config_from_env();
    pretty_env_logger::init_timed();
    let db = markov_bot::db::Db::new(config).await?;
    println!("{:?}", db.get_chat_id(1).await);
    Ok(())
}
