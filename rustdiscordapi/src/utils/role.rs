use reqwest::Client;
use serde_json::Value;
use std::error::Error;

/// Adds a role to a user in a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild.
/// * `user_id` - The ID of the user to add the role to.
/// * `role_id` - The ID of the role to add.
///
/// # Returns
///
/// A result indicating success or failure.
#[allow(dead_code)]
pub async fn add_role(client: &Client, token: &str, guild_id: &str, user_id: &str, role_id: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/members/{}/roles/{}", guild_id, user_id, role_id);
    
    client.put(&url)
        .bearer_auth(token)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}

/// Removes a role from a user in a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild.
/// * `user_id` - The ID of the user to remove the role from.
/// * `role_id` - The ID of the role to remove.
///
/// # Returns
///
/// A result indicating success or failure.
#[allow(dead_code)]
pub async fn remove_role(client: &Client, token: &str, guild_id: &str, user_id: &str, role_id: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/members/{}/roles/{}", guild_id, user_id, role_id);
    
    client.delete(&url)
        .bearer_auth(token)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}

/// Fetches information about a role in a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild.
/// * `role_id` - The ID of the role to fetch information for.
///
/// # Returns
///
/// A result containing the role information as a JSON value.
#[allow(dead_code)]
pub async fn fetch_role_info(client: &Client, token: &str, guild_id: &str, role_id: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/roles/{}", guild_id, role_id);
    let response: Value = client.get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}
