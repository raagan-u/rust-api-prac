use mongodb::bson::{doc, oid::ObjectId};
use serde::{
    de::{Deserializer, Error as SerdeError},
    {Deserialize, Serialize},
};

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
        }
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
    pub status: String, // Active, expired, closed
}
#[derive(Debug, Deserialize)]
pub struct PollRequest {
    pub poll_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub options: Vec<PollOption>,
    pub status: String,
    pub expiration_date: Option<chrono::DateTime<chrono::Utc>>,
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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_try_from_poll_request() {
        // Create a mock PollRequest
        let poll_request = PollRequest {
            poll_id: 1,
            title: "Sample Poll".to_string(),
            description: Some("This is a sample poll".to_string()),
            options: vec![
                PollOption {
                    text: "Option 1".to_string(),
                    votes: 0,
                },
                PollOption {
                    text: "Option 2".to_string(),
                    votes: 0,
                },
            ],
            expiration_date: Some(Utc.ymd(2024, 12, 31).and_hms(23, 59, 59)),
            status: "active".to_string(),
        };

        // Call try_from and unwrap the result
        let poll = Poll::try_from(poll_request).unwrap();

        // Assertions to ensure the conversion worked as expected
        assert_eq!(poll.poll_id, 1);
        assert_eq!(poll.title, "Sample Poll");
        assert_eq!(poll.description, Some("This is a sample poll".to_string()));

        // Assert that options were converted correctly
        assert_eq!(poll.options.len(), 2);
        assert_eq!(poll.options[0].text, "Option 1");
        assert_eq!(poll.options[0].votes, 0);
        assert_eq!(poll.options[1].text, "Option 2");
        assert_eq!(poll.options[1].votes, 0);

        // Assert the status
        assert_eq!(poll.status, "active");

        // Check if expiration date matches
        assert_eq!(
            poll.expiration_date,
            Some(Utc.ymd(2024, 12, 31).and_hms(23, 59, 59))
        );

        // Ensure created_at is set (rough check)
        assert!(poll.created_at <= Utc::now());
    }
}
