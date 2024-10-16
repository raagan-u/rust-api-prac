use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ObjectId, // User ID
    pub username: String,
    pub email: String,
    pub password: String,        // Store hashed password
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub polls_created: Vec<ObjectId>, // List of poll IDs created by the user
}
