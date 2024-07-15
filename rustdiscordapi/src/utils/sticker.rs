use reqwest::Client;
use serde_json::Value;
use std::error::Error;

#[allow(dead_code)]
/// Fetches a sticker by its ID.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `sticker_id` - The ID of the sticker to fetch.
///
/// # Returns
///
/// A result containing the sticker information as a JSON value.
pub async fn get_sticker(client: &Client, token: &str, sticker_id: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/stickers/{}", sticker_id);
    let response: Value = client.get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

#[allow(dead_code)]
/// Lists all standard sticker packs.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
///
/// # Returns
///
/// A result containing the list of sticker packs as a JSON value.
pub async fn list_sticker_packs(client: &Client, token: &str) -> Result<Value, Box<dyn Error>> {
    let url = "https://discord.com/api/v9/sticker-packs";
    let response: Value = client.get(url)
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

#[allow(dead_code)]
/// Lists all stickers for a guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild to list stickers for.
///
/// # Returns
///
/// A result containing the list of guild stickers as a JSON value.
pub async fn list_guild_stickers(client: &Client, token: &str, guild_id: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/stickers", guild_id);
    let response: Value = client.get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

#[allow(dead_code)]
/// Fetches a guild sticker by its ID.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild.
/// * `sticker_id` - The ID of the sticker to fetch.
///
/// # Returns
///
/// A result containing the guild sticker information as a JSON value.
pub async fn get_guild_sticker(client: &Client, token: &str, guild_id: &str, sticker_id: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/stickers/{}", guild_id, sticker_id);
    let response: Value = client.get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

#[allow(dead_code)]
/// Creates a new sticker in a guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild.
/// * `sticker_data` - The JSON value of the sticker data.
///
/// # Returns
///
/// A result containing the created guild sticker information as a JSON value.
pub async fn create_guild_sticker(client: &Client, token: &str, guild_id: &str, sticker_data: Value) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/stickers", guild_id);
    let response: Value = client.post(&url)
        .bearer_auth(token)
        .json(&sticker_data)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

#[allow(dead_code)]
/// Modifies a guild sticker.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild.
/// * `sticker_id` - The ID of the sticker to modify.
/// * `sticker_data` - The JSON value of the sticker data.
///
/// # Returns
///
/// A result containing the modified guild sticker information as a JSON value.
pub async fn modify_guild_sticker(client: &Client, token: &str, guild_id: &str, sticker_id: &str, sticker_data: Value) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/stickers/{}", guild_id, sticker_id);
    let response: Value = client.patch(&url)
        .bearer_auth(token)
        .json(&sticker_data)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

#[allow(dead_code)]
/// Deletes a guild sticker.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild.
/// * `sticker_id` - The ID of the sticker to delete.
///
/// # Returns
///
/// A result indicating success or failure.
pub async fn delete_guild_sticker(client: &Client, token: &str, guild_id: &str, sticker_id: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/stickers/{}", guild_id, sticker_id);
    
    client.delete(&url)
        .bearer_auth(token)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}
