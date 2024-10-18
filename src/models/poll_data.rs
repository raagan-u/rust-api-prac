use mongodb::bson::{doc, oid::ObjectId};
use serde::{{Deserialize, Serialize}, de::{Deserializer, Error as SerdeError}};

use chrono::{DateTime, Utc};

fn deserialize_date_time_opt<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let date_str: Option<String> = Option::deserialize(deserializer)?;
    
    match date_str {
        Some(s) => {
            // Try to parse the string into a DateTime<Utc>
            DateTime::parse_from_rfc3339(&s)
                .map(|dt| Some(dt.with_timezone(&Utc)))
                .map_err(D::Error::custom)
        },
        None => Ok(None), // Return None if no string is present
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PollOption {
    pub text: String, 
    pub votes: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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