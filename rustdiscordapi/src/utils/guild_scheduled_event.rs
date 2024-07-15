use reqwest::Client;
use serde_json::Value;
use std::error::Error;

#[allow(dead_code)]
/// Lists scheduled events for a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild to list scheduled events for.
///
/// # Returns
///
/// A result containing the list of scheduled events as a JSON value.
pub async fn list_scheduled_events(client: &Client, token: &str, guild_id: &str, event_id: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/scheduled-events/{}", guild_id, event_id);
    let response: Value = client.get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

#[allow(dead_code)]
/// Creates a scheduled event in a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild to create the scheduled event in.
/// * `event_settings` - The JSON value of the event settings.
///
/// # Returns
///
/// A result containing the created scheduled event information as a JSON value.
pub async fn create_scheduled_event(client: &Client, token: &str, guild_id: &str, event_settings: Value) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/scheduled-events", guild_id);
    let response: Value = client.post(&url)
        .bearer_auth(token)
        .json(&event_settings)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

#[allow(dead_code)]
/// Fetches a scheduled event from a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild.
/// * `event_id` - The ID of the scheduled event to fetch.
///
/// # Returns
///
/// A result containing the scheduled event information as a JSON value.
pub async fn get_scheduled_event(client: &Client, token: &str, guild_id: &str, event_id: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/scheduled-events/{}", guild_id, event_id);
    let response: Value = client.get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

#[allow(dead_code)]
/// Modifies a scheduled event in a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild.
/// * `event_id` - The ID of the scheduled event to modify.
/// * `event_settings` - The JSON value of the event settings.
///
/// # Returns
///
/// A result containing the modified scheduled event information as a JSON value.
pub async fn modify_scheduled_event(client: &Client, token: &str, guild_id: &str, event_id: &str, event_settings: Value) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/scheduled-events/{}", guild_id, event_id);
    let response: Value = client.patch(&url)
        .bearer_auth(token)
        .json(&event_settings)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

#[allow(dead_code)]
/// Deletes a scheduled event from a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild.
/// * `event_id` - The ID of the scheduled event to delete.
///
/// # Returns
///
/// A result indicating success or failure.
pub async fn delete_scheduled_event(client: &Client, token: &str, guild_id: &str, event_id: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/scheduled-events/{}", guild_id, event_id);
    
    client.delete(&url)
        .bearer_auth(token)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}

#[allow(dead_code)]
/// Fetches users of a scheduled event in a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild.
/// * `event_id` - The ID of the scheduled event to fetch users for.
///
/// # Returns
///
/// A result containing the list of users as a JSON value.
pub async fn get_scheduled_event_users(client: &Client, token: &str, guild_id: &str, event_id: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/scheduled-events/{}/users", guild_id, event_id);
    let response: Value = client.get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

#[allow(dead_code)]
/// Updates the status of a scheduled event in a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild.
/// * `event_id` - The ID of the scheduled event to update the status for.
/// * `status` - The new status of the scheduled event.
///
/// # Returns
///
/// A result containing the updated scheduled event information as a JSON value.
pub async fn update_scheduled_event_status(client: &Client, token: &str, guild_id: &str, event_id: &str, status: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/scheduled-events/{}", guild_id, event_id);
    let body = serde_json::json!({ "status": status });
    let response: Value = client.patch(&url)
        .bearer_auth(token)
        .json(&body)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

#[allow(dead_code)]
/// Fetches the permissions required for guild scheduled events.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
///
/// # Returns
///
/// A result containing the permissions requirements as a JSON value.
pub async fn get_scheduled_event_permissions(client: &Client, token: &str) -> Result<Value, Box<dyn Error>> {
    let url = "https://discord.com/api/v9/scheduled-events/permissions";
    let response: Value = client.get(url)
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}