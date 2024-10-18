use crate::models::poll_data::Poll;
#[async_trait::async_trait]
pub trait PollRepository: Send + Sync {
	async fn create_poll(&self, poll: Poll) -> Result<Poll, Box<dyn std::error::Error>>;
    async fn get_poll(&self) -> Result<Vec<Poll>, Box<dyn std::error::Error>>;
    async fn update_poll(&self, poll: Poll) -> Result<Poll, Box<dyn std::error::Error>>;
    async fn delete_poll(&self, poll_id: i64) -> Result<(), Box<dyn std::error::Error>>;
}