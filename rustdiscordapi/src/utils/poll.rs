use reqwest::Client;
use serde_json::Value;
use std::error::Error;

#[allow(dead_code)]
/// Fetches the voters for a specific answer in a poll.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `channel_id` - The ID of the channel.
/// * `message_id` - The ID of the poll message.
/// * `answer_id` - The ID of the answer to fetch voters for.
/// * `after` - (Optional) Fetch users after this user ID.
/// * `limit` - (Optional) Max number of users to return (1-100).
///
/// # Returns
///
/// A result containing the list of voters as a JSON value.
pub async fn get_answer_voters(client: &Client, token: &str, channel_id: &str, message_id: &str, answer_id: &str, after: Option<&str>, limit: Option<u32>) -> Result<Value, Box<dyn Error>> {
    let mut url = format!("https://discord.com/api/v9/channels/{}/polls/{}/answers/{}/voters", channel_id, message_id, answer_id);
    
    if after.is_some() || limit.is_some() {
        url.push('?');
        if let Some(after) = after {
            url.push_str(&format!("after={}&", after));
        }
        if let Some(limit) = limit {
            url.push_str(&format!("limit={}&", limit));
        }
        url.pop(); // Remove trailing '&' or '?'.
    }
    
    let response: Value = client.get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

#[allow(dead_code)]
/// Ends a poll.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `channel_id` - The ID of the channel.
/// * `message_id` - The ID of the poll message.
///
/// # Returns
///
/// A result containing the updated message information as a JSON value.
pub async fn end_poll(client: &Client, token: &str, channel_id: &str, message_id: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/channels/{}/polls/{}/expire", channel_id, message_id);
    let response: Value = client.post(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}
