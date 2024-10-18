pub mod config;
use crate::poll_repo::{poll_repo::PollRepository, mongo_poll_repo::MongoPollRepo};

use config::DbConfig;


pub async fn init(config: DbConfig) -> Box<dyn PollRepository> {
    match config.db_type.as_str() {
        "mongodb" => {
            let repo = MongoPollRepo::new(&config).await; // Await the Future to get MongoPollRepo
            Box::new(repo) // Box the resulting MongoPollRepo
        }
        _ => panic!("Unsupported database type"),
    }
}