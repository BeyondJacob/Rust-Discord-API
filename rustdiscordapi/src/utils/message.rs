use reqwest::Client;
use serde_json::json;
use std::error::Error;

/// Sends a message to a specified Discord channel.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `channel_id` - The ID of the channel to send the message to.
/// * `content` - The content of the message.
///
/// # Returns
///
/// A result indicating success or failure.
#[allow(dead_code)]
pub async fn send_message(client: &Client, token: &str, channel_id: &str, content: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/channels/{}/messages", channel_id);
    let body = json!({ "content": content });
    
    client.post(&url)
        .bearer_auth(token)
        .json(&body)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}

/// Edits a message in a specified Discord channel.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `channel_id` - The ID of the channel where the message is located.
/// * `message_id` - The ID of the message to edit.
/// * `new_content` - The new content of the message.
///
/// # Returns
///
/// A result indicating success or failure.
#[allow(dead_code)]
pub async fn edit_message(client: &Client, token: &str, channel_id: &str, message_id: &str, new_content: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/channels/{}/messages/{}", channel_id, message_id);
    let body = json!({ "content": new_content });
    
    client.patch(&url)
        .bearer_auth(token)
        .json(&body)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}

/// Deletes a message in a specified Discord channel.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `channel_id` - The ID of the channel where the message is located.
/// * `message_id` - The ID of the message to delete.
///
/// # Returns
///
/// A result indicating success or failure.
#[allow(dead_code)]
pub async fn delete_message(client: &Client, token: &str, channel_id: &str, message_id: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/channels/{}/messages/{}", channel_id, message_id);
    
    client.delete(&url)
        .bearer_auth(token)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}

/// Pins a message in a specified Discord channel.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `channel_id` - The ID of the channel where the message is located.
/// * `message_id` - The ID of the message to pin.
///
/// # Returns
///
/// A result indicating success or failure.
#[allow(dead_code)]
pub async fn pin_message(client: &Client, token: &str, channel_id: &str, message_id: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/channels/{}/pins/{}", channel_id, message_id);
    
    client.put(&url)
        .bearer_auth(token)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}

/// Unpins a message in a specified Discord channel.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `channel_id` - The ID of the channel where the message is located.
/// * `message_id` - The ID of the message to unpin.
///
/// # Returns
///
/// A result indicating success or failure.
#[allow(dead_code)]
pub async fn unpin_message(client: &Client, token: &str, channel_id: &str, message_id: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/channels/{}/pins/{}", channel_id, message_id);
    
    client.delete(&url)
        .bearer_auth(token)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}
