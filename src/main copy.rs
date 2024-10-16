mod db;
mod models;
use db::connection::get_db;
use models::poll_data::{Poll, PollOption};
use mongodb::Collection;
use mongodb::bson::oid::ObjectId;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = get_db().await?;
    let polls_collection: Collection<Poll> = db.collection("polls");

    // Example: Create a poll
    let poll = Poll {
        id: ObjectId::new(),
        title: String::from("What's your favorite programming language?"),
        description: Some(String::from("Choose one from the list below")),
        options: vec![
            PollOption { id: ObjectId::new(), text: String::from("Rust"), votes: 0 },
            PollOption { id: ObjectId::new(), text: String::from("JavaScript"), votes: 0 },
            PollOption { id: ObjectId::new(), text: String::from("Python"), votes: 0 },
        ],
        created_by: ObjectId::new(),
        created_at: chrono::Utc::now(),
        expiration_date: None,
        status: String::from("active"),
    };

    polls_collection.insert_one(poll, None).await?;
    println!("Poll created successfully!");

    Ok(())
}
