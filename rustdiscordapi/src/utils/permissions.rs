use reqwest::Client;
use serde_json::Value;
use std::error::Error;

/// Checks if a user has a specific permission in a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild.
/// * `user_id` - The ID of the user.
/// * `permission` - The permission to check for.
///
/// # Returns
///
/// A result indicating whether the user has the specified permission.
#[allow(dead_code)]
pub async fn check_permission(client: &Client, token: &str, guild_id: &str, user_id: &str, permission: &str) -> Result<bool, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/members/{}", guild_id, user_id);
    let response: Value = client.get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;
    
    if let Some(permissions) = response["permissions"].as_str() {
        // This is a simplified check. Adjust based on your specific permission needs.
        Ok(permissions.contains(permission))
    } else {
        Ok(false)
    }
}
