use mongodb::bson::{doc, oid::ObjectId, DateTime as BsonDateTime};
use serde::{{Deserialize, Serialize}, de::{self, Deserializer}};

use chrono::{DateTime, TimeZone, Utc};

fn deserialize_date_time_opt<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let bson_date: Option<BsonDateTime> = Option::deserialize(deserializer)?;

    if let Some(bson_date_time) = bson_date {
        // Get the timestamp in milliseconds and convert it to chrono::DateTime<Utc>
        let timestamp_millis = bson_date_time.timestamp_millis();
        
        // Use `timestamp_millis_opt` instead of `timestamp_millis` to handle errors
        match Utc.timestamp_millis_opt(timestamp_millis).single() {
            Some(datetime) => Ok(Some(datetime)),
            None => Err(de::Error::custom("Invalid or ambiguous timestamp")),
        }
    } else {
        Ok(None) // Return None if BSON date is not present
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PollOption {
    pub id: ObjectId, 
    pub text: String, 
    pub votes: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Poll {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub poll_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub options: Vec<PollOption>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(deserialize_with = "deserialize_date_time_opt")]
    pub expiration_date: Option<chrono::DateTime<chrono::Utc>>,
    pub status: String,           // Active, expired, closed
}
#[derive(Debug, Deserialize)]
pub struct PollRequest {
    pub poll_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub options: Vec<PollOption>,
    pub status: String,
    pub expiration_date: Option<chrono::DateTime<chrono::Utc>>
}

impl TryFrom<PollRequest> for Poll {
    type Error = Box<dyn std::error::Error>;

    fn try_from(item: PollRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            id: ObjectId::new(),
            poll_id: item.poll_id,
            title: item.title,
            description: item.description,
            options: item.options,
            created_at: chrono::Utc::now(),
            expiration_date: item.expiration_date,
            status: item.status,
        })
    }
}