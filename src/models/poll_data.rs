use mongodb::bson::{doc, oid::ObjectId};
use serde::{{Deserialize, Serialize}, de::Deserializer};

use chrono::{DateTime, NaiveDateTime, Utc};


fn deserialize_date_time_opt<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(date_str) => {
            // Parse the string into NaiveDateTime, and then convert to DateTime<Utc>
            let naive_date = NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M:%S")
                .map_err(serde::de::Error::custom)?;
            // Create DateTime<Utc> from NaiveDateTime
           // let utc_date = DateTime::from_utc(naive_date, Utc);
           let utc_date = naive_date.and_utc();
            Ok(Some(utc_date))
        }
        None => Ok(None),
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