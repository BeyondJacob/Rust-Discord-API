use reqwest::Client;
use serde_json::Value;
use std::error::Error;

#[allow(dead_code)]
/// Fetches the current user's information.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
///
/// # Returns
///
/// A result containing the current user's information as a JSON value.
pub async fn get_current_user(client: &Client, token: &str) -> Result<Value, Box<dyn Error>> {
    let url = "https://discord.com/api/v9/users/@me";
    let response: Value = client.get(url)
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

#[allow(dead_code)]
/// Fetches a user's information.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `user_id` - The ID of the user to fetch.
///
/// # Returns
///
/// A result containing the user's information as a JSON value.
pub async fn get_user(client: &Client, token: &str, user_id: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/users/{}", user_id);
    let response: Value = client.get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

#[allow(dead_code)]
/// Modifies the current user's information.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `settings` - The JSON value of the settings to update.
///
/// # Returns
///
/// A result indicating success or failure.
pub async fn modify_current_user(client: &Client, token: &str, settings: Value) -> Result<Value, Box<dyn Error>> {
    let url = "https://discord.com/api/v9/users/@me";
    let response: Value = client.patch(url)
        .bearer_auth(token)
        .json(&settings)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

#[allow(dead_code)]
/// Fetches the current user's guilds.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
///
/// # Returns
///
/// A result containing the current user's guilds as a JSON value.
pub async fn get_current_user_guilds(client: &Client, token: &str) -> Result<Value, Box<dyn Error>> {
    let url = "https://discord.com/api/v9/users/@me/guilds";
    let response: Value = client.get(url)
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

#[allow(dead_code)]
/// Fetches the current user's guild member information for a specific guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild to fetch member information for.
///
/// # Returns
///
/// A result containing the guild member information as a JSON value.
pub async fn get_current_user_guild_member(client: &Client, token: &str, guild_id: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/users/@me/guilds/{}/member", guild_id);
    let response: Value = client.get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

#[allow(dead_code)]
/// Leaves a guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild to leave.
///
/// # Returns
///
/// A result indicating success or failure.
pub async fn leave_guild(client: &Client, token: &str, guild_id: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/users/@me/guilds/{}", guild_id);
    
    client.delete(&url)
        .bearer_auth(token)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}

#[allow(dead_code)]
/// Creates a DM channel with a user.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `recipient_id` - The ID of the user to create a DM with.
///
/// # Returns
///
/// A result containing the created DM channel information as a JSON value.
pub async fn create_dm(client: &Client, token: &str, recipient_id: &str) -> Result<Value, Box<dyn Error>> {
    let url = "https://discord.com/api/v9/users/@me/channels";
    let body = serde_json::json!({ "recipient_id": recipient_id });
    let response: Value = client.post(url)
        .bearer_auth(token)
        .json(&body)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

#[allow(dead_code)]
/// Creates a group DM channel.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `access_tokens` - The OAuth2 access tokens of the users to add.
/// * `nicks` - The nicknames of the users to add.
///
/// # Returns
///
/// A result containing the created group DM channel information as a JSON value.
pub async fn create_group_dm(client: &Client, token: &str, access_tokens: Vec<&str>, nicks: Value) -> Result<Value, Box<dyn Error>> {
    let url = "https://discord.com/api/v9/users/@me/channels";
    let body = serde_json::json!({ "access_tokens": access_tokens, "nicks": nicks });
    let response: Value = client.post(url)
        .bearer_auth(token)
        .json(&body)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

#[allow(dead_code)]
/// Fetches the current user's connections.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
///
/// # Returns
///
/// A result containing the current user's connections as a JSON value.
pub async fn get_current_user_connections(client: &Client, token: &str) -> Result<Value, Box<dyn Error>> {
    let url = "https://discord.com/api/v9/users/@me/connections";
    let response: Value = client.get(url)
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

#[allow(dead_code)]
/// Fetches the current user's application role connection.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `application_id` - The ID of the application to fetch role connection for.
///
/// # Returns
///
/// A result containing the role connection information as a JSON value.
pub async fn get_current_user_application_role_connection(client: &Client, token: &str, application_id: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/users/@me/applications/{}/role-connection", application_id);
    let response: Value = client.get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

#[allow(dead_code)]
/// Updates the current user's application role connection.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `application_id` - The ID of the application to update role connection for.
/// * `role_connection` - The JSON value of the role connection settings.
///
/// # Returns
///
/// A result indicating success or failure.
pub async fn update_current_user_application_role_connection(client: &Client, token: &str, application_id: &str, role_connection: Value) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/users/@me/applications/{}/role-connection", application_id);
    let response: Value = client.put(&url)
        .bearer_auth(token)
        .json(&role_connection)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

#[allow(dead_code)]
/// Kicks a user from a guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild.
/// * `user_id` - The ID of the user to kick.
///
/// # Returns
///
/// A result indicating success or failure.
pub async fn kick_user(client: &Client, token: &str, guild_id: &str, user_id: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/members/{}", guild_id, user_id);
    
    client.delete(&url)
        .bearer_auth(token)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}

#[allow(dead_code)]
/// Bans a user from a guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild.
/// * `user_id` - The ID of the user to ban.
/// * `delete_message_days` - The number of days to delete messages for (0-7).
/// * `reason` - The reason for the ban.
///
/// # Returns
///
/// A result indicating success or failure.
pub async fn ban_user(client: &Client, token: &str, guild_id: &str, user_id: &str, delete_message_days: u8, reason: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/bans/{}", guild_id, user_id);
    let body = serde_json::json!({
        "delete_message_days": delete_message_days,
        "reason": reason
    });
    
    client.put(&url)
        .bearer_auth(token)
        .json(&body)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}