use crate::poll_repo::poll_repo::PollRepository;
use crate::models::poll_data::Poll;
use crate::db::config::DbConfig;
use futures::StreamExt;

use mongodb::bson::{doc, to_document};
use mongodb::{Client, Collection};

#[derive(Clone)]
pub struct MongoPollRepo {
	collection: Collection<Poll>,
}

impl MongoPollRepo {
    pub async fn new(config: &DbConfig) -> Self {
        // Create a MongoDB client
        let client = Client::with_uri_str(&config.connection_string)
            .await
            .expect("Failed to initialize MongoDB client");

        // Get the specified database and collection
        let database = client.database(&config.database_name);
        let collection = database.collection::<Poll>("Poll");

        MongoPollRepo { collection }
    }
}

#[async_trait::async_trait]
impl PollRepository for MongoPollRepo {
	async fn create_poll(&self, poll: Poll) -> Result<Poll, Box<dyn std::error::Error>> {
        println!("Entered Create Poll Func");
		let result = self.collection.insert_one(poll.clone(), None).await?;
    
		// Get the ObjectId of the newly inserted poll
		let inserted_id = result.inserted_id.as_object_id().ok_or_else(|| {
			mongodb::error::Error::from(std::io::Error::new(
				std::io::ErrorKind::Other,
				"Failed to get inserted ObjectId",
			))
		})?;
		
		// Return the poll with the new ObjectId
		Ok(Poll {
			id: inserted_id,
			..poll
		})
    }

    async fn get_poll(&self) -> Result<Vec<Poll>, Box<dyn std::error::Error>> {
        println!("Entered get_poll");
        let mut cursor = self.collection.find(None, None).await?;
        let mut polls = Vec::new();
        
        while let Some(result) = cursor.next().await {
            match result {
                Ok(document) => {
                    polls.push(document);
                }
                Err(e) => eprintln!("Error retrieving polls: {:?}", e),
            }
        }
        Ok(polls)
    }

    async fn update_poll(&self, poll: Poll) -> Result<Poll, Box<dyn std::error::Error>> {
        let filter = doc! { "_id": &poll.id };
        let update = doc! { "$set": to_document(&poll).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)? };
        self.collection.update_one(filter, update, None).await?;
        Ok(poll) // Return the updated poll
    }

    async fn delete_poll(&self, poll_id: i64) -> Result<(), Box<dyn std::error::Error>> {
        let filter = doc! { "poll_id": poll_id };
        self.collection.delete_one(filter, None).await?;
        Ok(())
    }
}