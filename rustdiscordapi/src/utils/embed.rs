use reqwest::Client;
use serde_json::json;
use std::error::Error;

/// Sends an embed message to a specified Discord channel.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `channel_id` - The ID of the channel to send the embed message to.
/// * `title` - The title of the embed.
/// * `description` - The description of the embed.
///
/// # Returns
///
/// A result indicating success or failure.
#[allow(dead_code)]
pub async fn send_embed_message(client: &Client, token: &str, channel_id: &str, title: &str, description: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/channels/{}/messages", channel_id);
    let embed = json!({
        "title": title,
        "description": description,
        "color": 0x3498db // Example color
    });
    let body = json!({ "embed": embed });
    
    client.post(&url)
        .bearer_auth(token)
        .json(&body)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}
