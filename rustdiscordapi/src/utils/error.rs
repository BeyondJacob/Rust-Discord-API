use reqwest::Client;
use serde_json::json;
use std::error::Error;

#[allow(dead_code)]
/// Sends an error message to a specified Discord channel.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `channel_id` - The ID of the channel to send the error message to.
/// * `error_message` - The content of the error message.
///
/// # Returns
///
/// A result indicating success or failure.
pub async fn send_error_message(client: &Client, token: &str, channel_id: &str, error_message: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/channels/{}/messages", channel_id);
    let body = json!({ "content": format!("Error: {}", error_message) });
    
    client.post(&url)
        .bearer_auth(token)
        .json(&body)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}
