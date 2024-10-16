use std::env;
use dotenv::dotenv;

use futures::StreamExt;
use mongodb::{
    bson::{doc, to_bson, Bson},
    results::{InsertOneResult, UpdateResult, DeleteResult},
    Client, Collection,
};
use bson::DateTime as BsonDateTime;
use std::time::SystemTime;
use crate::models::poll_data::Poll;

pub struct Database {
    poll: Collection<Poll>,
}

impl Database {
    pub async fn init() -> Self {
        dotenv().ok();

        let uri = env::var("MONGODB_URI").unwrap_or_else(|_| {
            "mongodb://localhost:27017/?directConnection=true".to_string()
        });

        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("rustest");

        let poll: Collection<Poll> = db.collection("Poll");

        Database {
            poll,
        }
    }

	pub async fn create_poll(&self, poll: Poll) -> Result<InsertOneResult, mongodb::error::Error>{
		let result = self
			.poll
			.insert_one(poll, None).await?;
			//.await
			//.ok()
			//.expect("Error creating Poll");
		Ok(result)
	}

    pub async fn get_poll(&self) -> Result<Vec<Poll>, mongodb::error::Error>{
        let mut cursor = self
            .poll
            .find(None, None).await?;
        let mut polls = Vec::new();
        while let Some(poll) = cursor.next().await {
            match poll {
                Ok(document) => polls.push(document),
                Err(e) => eprintln!("Error Retrieveing Polls {:?} ", e)
            }
        }
        Ok(polls)
    }

    pub async fn update_poll(&self, poll: Poll) -> Result<UpdateResult, mongodb::error::Error> {
        let options_bson = to_bson(&poll.options)?;
        
        let filter = doc! { "poll_id": poll.poll_id }; // Filter by poll_id
        let update = doc! {
            "$set": {
                "title": poll.title,
                "description": poll.description,
                "options": options_bson,
                "status": poll.status,
                "expiration_date": match poll.expiration_date {
                    Some(date_time) => {
                        let system_time: SystemTime = date_time.into();
                        Bson::DateTime(BsonDateTime::from(system_time)) // Correct conversion to Bson
                    },
                    None => Bson::Null, // Handle None Case
                },
            }
        };
        let result = self
            .poll
            .update_one(filter, update, None)
            .await?;
            
        Ok(result)
    }

    pub async fn delete_poll(&self, poll_id: i64) -> Result<DeleteResult, mongodb::error::Error> {
        let filter = doc! { "poll_id": poll_id };
        let result = self.poll.delete_one(filter, None).await?;
        Ok(result)
    }

}