use reqwest::Client;
use serde_json::Value;
use std::error::Error;

/// Creates a new Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_settings` - The JSON value of the guild settings.
///
/// # Returns
///
/// A result containing the created guild information as a JSON value.
#[allow(dead_code)]
pub async fn create_guild(client: &Client, token: &str, guild_settings: Value) -> Result<Value, Box<dyn Error>> {
    let url = "https://discord.com/api/v9/guilds";
    let response: Value = client.post(url)
        .bearer_auth(token)
        .json(&guild_settings)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

/// Fetches information about a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild to fetch information for.
///
/// # Returns
///
/// A result containing the guild information as a JSON value.
#[allow(dead_code)]
pub async fn get_guild(client: &Client, token: &str, guild_id: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}", guild_id);
    let response: Value = client.get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

/// Fetches a preview of a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild to fetch a preview for.
///
/// # Returns
///
/// A result containing the guild preview as a JSON value.
#[allow(dead_code)]
pub async fn get_guild_preview(client: &Client, token: &str, guild_id: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/preview", guild_id);
    let response: Value = client.get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

/// Modifies a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild to modify.
/// * `settings` - The JSON value of the settings to update.
///
/// # Returns
///
/// A result indicating success or failure.
#[allow(dead_code)]
pub async fn modify_guild(client: &Client, token: &str, guild_id: &str, settings: Value) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}", guild_id);
    
    client.patch(&url)
        .bearer_auth(token)
        .json(&settings)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}

/// Deletes a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild to delete.
///
/// # Returns
///
/// A result indicating success or failure.
#[allow(dead_code)]
pub async fn delete_guild(client: &Client, token: &str, guild_id: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}", guild_id);
    
    client.delete(&url)
        .bearer_auth(token)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}

/// Fetches channels of a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild to fetch channels for.
///
/// # Returns
///
/// A result containing the guild channels as a JSON value.
#[allow(dead_code)]
pub async fn get_guild_channels(client: &Client, token: &str, guild_id: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/channels", guild_id);
    let response: Value = client.get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

/// Creates a new channel in a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild to create the channel in.
/// * `channel_settings` - The JSON value of the channel settings.
///
/// # Returns
///
/// A result containing the created channel information as a JSON value.
#[allow(dead_code)]
pub async fn create_guild_channel(client: &Client, token: &str, guild_id: &str, channel_settings: Value) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/channels", guild_id);
    let response: Value = client.post(&url)
        .bearer_auth(token)
        .json(&channel_settings)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

/// Modifies the positions of channels in a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild to modify channel positions for.
/// * `positions` - The JSON value of the new positions.
///
/// # Returns
///
/// A result indicating success or failure.
#[allow(dead_code)]
pub async fn modify_guild_channel_positions(client: &Client, token: &str, guild_id: &str, positions: Value) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/channels", guild_id);
    
    client.patch(&url)
        .bearer_auth(token)
        .json(&positions)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}

/// Lists active threads in a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild to list active threads for.
///
/// # Returns
///
/// A result containing the list of active threads as a JSON value.
#[allow(dead_code)]
pub async fn list_active_guild_threads(client: &Client, token: &str, guild_id: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/threads/active", guild_id);
    let response: Value = client.get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

/// Fetches a member of a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild.
/// * `user_id` - The ID of the member to fetch.
///
/// # Returns
///
/// A result containing the member information as a JSON value.
#[allow(dead_code)]
pub async fn get_guild_member(client: &Client, token: &str, guild_id: &str, user_id: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/members/{}", guild_id, user_id);
    let response: Value = client.get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

/// Lists members of a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild.
///
/// # Returns
///
/// A result containing the list of members as a JSON value.
#[allow(dead_code)]
pub async fn list_guild_members(client: &Client, token: &str, guild_id: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/members", guild_id);
    let response: Value = client.get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

/// Searches for members in a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild.
/// * `query` - The search query string.
///
/// # Returns
///
/// A result containing the search results as a JSON value.
#[allow(dead_code)]
pub async fn search_guild_members(client: &Client, token: &str, guild_id: &str, query: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/members/search?query={}", guild_id, query);
    let response: Value = client.get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

/// Adds a member to a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild.
/// * `user_id` - The ID of the user to add.
/// * `member_settings` - The JSON value of the member settings.
///
/// # Returns
///
/// A result containing the added member information as a JSON value.
#[allow(dead_code)]
pub async fn add_guild_member(client: &Client, token: &str, guild_id: &str, user_id: &str, member_settings: Value) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/members/{}", guild_id, user_id);
    let response: Value = client.put(&url)
        .bearer_auth(token)
        .json(&member_settings)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

/// Modifies a member in a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild.
/// * `user_id` - The ID of the member to modify.
/// * `settings` - The JSON value of the settings to update.
///
/// # Returns
///
/// A result indicating success or failure.
#[allow(dead_code)]
pub async fn modify_guild_member(client: &Client, token: &str, guild_id: &str, user_id: &str, settings: Value) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/members/{}", guild_id, user_id);
    
    client.patch(&url)
        .bearer_auth(token)
        .json(&settings)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}

/// Modifies the current member in a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild.
/// * `settings` - The JSON value of the settings to update.
///
/// # Returns
///
/// A result indicating success or failure.
#[allow(dead_code)]
pub async fn modify_current_member(client: &Client, token: &str, guild_id: &str, settings: Value) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/members/@me", guild_id);
    
    client.patch(&url)
        .bearer_auth(token)
        .json(&settings)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}

/// Modifies the current user's nickname in a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild.
/// * `nick` - The new nickname.
///
/// # Returns
///
/// A result indicating success or failure.
#[allow(dead_code)]
pub async fn modify_current_user_nick(client: &Client, token: &str, guild_id: &str, nick: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/members/@me/nick", guild_id);
    let body = serde_json::json!({ "nick": nick });
    
    client.patch(&url)
        .bearer_auth(token)
        .json(&body)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}

/// Adds a role to a member in a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild.
/// * `user_id` - The ID of the member to add the role to.
/// * `role_id` - The ID of the role to add.
///
/// # Returns
///
/// A result indicating success or failure.
#[allow(dead_code)]
pub async fn add_guild_member_role(client: &Client, token: &str, guild_id: &str, user_id: &str, role_id: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/members/{}/roles/{}", guild_id, user_id, role_id);
    
    client.put(&url)
        .bearer_auth(token)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}

/// Removes a role from a member in a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild.
/// * `user_id` - The ID of the member to remove the role from.
/// * `role_id` - The ID of the role to remove.
///
/// # Returns
///
/// A result indicating success or failure.
#[allow(dead_code)]
pub async fn remove_guild_member_role(client: &Client, token: &str, guild_id: &str, user_id: &str, role_id: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/members/{}/roles/{}", guild_id, user_id, role_id);
    
    client.delete(&url)
        .bearer_auth(token)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}

/// Removes a member from a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild.
/// * `user_id` - The ID of the member to remove.
///
/// # Returns
///
/// A result indicating success or failure.
#[allow(dead_code)]
pub async fn remove_guild_member(client: &Client, token: &str, guild_id: &str, user_id: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/members/{}", guild_id, user_id);
    
    client.delete(&url)
        .bearer_auth(token)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}

/// Fetches bans of a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild to fetch bans for.
///
/// # Returns
///
/// A result containing the guild bans as a JSON value.
#[allow(dead_code)]
pub async fn get_guild_bans(client: &Client, token: &str, guild_id: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/bans", guild_id);
    let response: Value = client.get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

/// Fetches a specific ban in a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild.
/// * `user_id` - The ID of the user to fetch the ban for.
///
/// # Returns
///
/// A result containing the ban information as a JSON value.
#[allow(dead_code)]
pub async fn get_guild_ban(client: &Client, token: &str, guild_id: &str, user_id: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/bans/{}", guild_id, user_id);
    let response: Value = client.get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

/// Creates a ban in a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild.
/// * `user_id` - The ID of the user to ban.
/// * `ban_settings` - The JSON value of the ban settings.
///
/// # Returns
///
/// A result indicating success or failure.
#[allow(dead_code)]
pub async fn create_guild_ban(client: &Client, token: &str, guild_id: &str, user_id: &str, ban_settings: Value) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/bans/{}", guild_id, user_id);
    
    client.put(&url)
        .bearer_auth(token)
        .json(&ban_settings)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}

/// Removes a ban in a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild.
/// * `user_id` - The ID of the user to unban.
///
/// # Returns
///
/// A result indicating success or failure.
#[allow(dead_code)]
pub async fn remove_guild_ban(client: &Client, token: &str, guild_id: &str, user_id: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/bans/{}", guild_id, user_id);
    
    client.delete(&url)
        .bearer_auth(token)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}

/// Bulk bans users in a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild.
/// * `user_ids` - A list of user IDs to ban.
///
/// # Returns
///
/// A result indicating success or failure.
#[allow(dead_code)]
pub async fn bulk_guild_ban(client: &Client, token: &str, guild_id: &str, user_ids: Vec<&str>) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/bans", guild_id);
    let body = serde_json::json!({ "user_ids": user_ids });
    
    client.post(&url)
        .bearer_auth(token)
        .json(&body)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}

/// Fetches roles of a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild to fetch roles for.
///
/// # Returns
///
/// A result containing the guild roles as a JSON value.
#[allow(dead_code)]
pub async fn get_guild_roles(client: &Client, token: &str, guild_id: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/roles", guild_id);
    let response: Value = client.get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

/// Creates a new role in a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild to create the role in.
/// * `role_settings` - The JSON value of the role settings.
///
/// # Returns
///
/// A result containing the created role information as a JSON value.
#[allow(dead_code)]
pub async fn create_guild_role(client: &Client, token: &str, guild_id: &str, role_settings: Value) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/roles", guild_id);
    let response: Value = client.post(&url)
        .bearer_auth(token)
        .json(&role_settings)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

/// Modifies the positions of roles in a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild to modify role positions for.
/// * `positions` - The JSON value of the new positions.
///
/// # Returns
///
/// A result indicating success or failure.
#[allow(dead_code)]
pub async fn modify_guild_role_positions(client: &Client, token: &str, guild_id: &str, positions: Value) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/roles", guild_id);
    
    client.patch(&url)
        .bearer_auth(token)
        .json(&positions)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}

/// Modifies a role in a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild.
/// * `role_id` - The ID of the role to modify.
/// * `settings` - The JSON value of the settings to update.
///
/// # Returns
///
/// A result indicating success or failure.
#[allow(dead_code)]
pub async fn modify_guild_role(client: &Client, token: &str, guild_id: &str, role_id: &str, settings: Value) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/roles/{}", guild_id, role_id);
    
    client.patch(&url)
        .bearer_auth(token)
        .json(&settings)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}

/// Modifies the MFA level of a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild.
/// * `level` - The new MFA level.
///
/// # Returns
///
/// A result indicating success or failure.
#[allow(dead_code)]
pub async fn modify_guild_mfa_level(client: &Client, token: &str, guild_id: &str, level: u8) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/mfa", guild_id);
    let body = serde_json::json!({ "level": level });
    
    client.post(&url)
        .bearer_auth(token)
        .json(&body)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}

/// Deletes a role from a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild.
/// * `role_id` - The ID of the role to delete.
///
/// # Returns
///
/// A result indicating success or failure.
#[allow(dead_code)]
pub async fn delete_guild_role(client: &Client, token: &str, guild_id: &str, role_id: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/roles/{}", guild_id, role_id);
    
    client.delete(&url)
        .bearer_auth(token)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}

/// Fetches the prune count of a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild to fetch prune count for.
/// * `days` - The number of days to count members without activity.
///
/// # Returns
///
/// A result containing the prune count as a JSON value.
#[allow(dead_code)]
pub async fn get_guild_prune_count(client: &Client, token: &str, guild_id: &str, days: u8) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/prune?days={}", guild_id, days);
    let response: Value = client.get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

/// Begins pruning members in a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild to prune members in.
/// * `days` - The number of days to count members without activity.
///
/// # Returns
///
/// A result containing the prune count as a JSON value.
#[allow(dead_code)]
pub async fn begin_guild_prune(client: &Client, token: &str, guild_id: &str, days: u8) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/prune", guild_id);
    let body = serde_json::json!({ "days": days });
    let response: Value = client.post(&url)
        .bearer_auth(token)
        .json(&body)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

/// Fetches voice regions of a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild to fetch voice regions for.
///
/// # Returns
///
/// A result containing the voice regions as a JSON value.
#[allow(dead_code)]
pub async fn get_guild_voice_regions(client: &Client, token: &str, guild_id: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/regions", guild_id);
    let response: Value = client.get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

/// Fetches invites of a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild to fetch invites for.
///
/// # Returns
///
/// A result containing the guild invites as a JSON value.
#[allow(dead_code)]
pub async fn get_guild_invites(client: &Client, token: &str, guild_id: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/invites", guild_id);
    let response: Value = client.get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

/// Fetches integrations of a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild to fetch integrations for.
///
/// # Returns
///
/// A result containing the guild integrations as a JSON value.
#[allow(dead_code)]
pub async fn get_guild_integrations(client: &Client, token: &str, guild_id: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/integrations", guild_id);
    let response: Value = client.get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

/// Deletes an integration from a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild.
/// * `integration_id` - The ID of the integration to delete.
///
/// # Returns
///
/// A result indicating success or failure.
#[allow(dead_code)]
pub async fn delete_guild_integration(client: &Client, token: &str, guild_id: &str, integration_id: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/integrations/{}", guild_id, integration_id);
    
    client.delete(&url)
        .bearer_auth(token)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}

/// Fetches widget settings of a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild to fetch widget settings for.
///
/// # Returns
///
/// A result containing the widget settings as a JSON value.
#[allow(dead_code)]
pub async fn get_guild_widget_settings(client: &Client, token: &str, guild_id: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/widget", guild_id);
    let response: Value = client.get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

/// Modifies widget settings of a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild to modify widget settings for.
/// * `settings` - The JSON value of the settings to update.
///
/// # Returns
///
/// A result indicating success or failure.
#[allow(dead_code)]
pub async fn modify_guild_widget_settings(client: &Client, token: &str, guild_id: &str, settings: Value) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/widget", guild_id);
    
    client.patch(&url)
        .bearer_auth(token)
        .json(&settings)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}

/// Fetches the widget of a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild to fetch the widget for.
///
/// # Returns
///
/// A result containing the guild widget as a JSON value.
#[allow(dead_code)]
#[allow(unused_variables)]  // Token is 'unused', crate identified.
pub async fn get_guild_widget(client: &Client, token: &str, guild_id: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/widget.json", guild_id);
    let response: Value = client.get(&url)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

/// Fetches the vanity URL of a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild to fetch the vanity URL for.
///
/// # Returns
///
/// A result containing the vanity URL as a JSON value.
#[allow(dead_code)]
pub async fn get_guild_vanity_url(client: &Client, token: &str, guild_id: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/vanity-url", guild_id);
    let response: Value = client.get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

/// Fetches the widget image of a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild to fetch the widget image for.
///
/// # Returns
///
/// A result containing the widget image as a JSON value.
#[allow(dead_code)]
#[allow(unused_variables)] // Token is 'unused', crate identified.
pub async fn get_guild_widget_image(client: &Client, token: &str, guild_id: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/widget.png", guild_id);
    let response: Value = client.get(&url)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

/// Fetches the welcome screen of a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild to fetch the welcome screen for.
///
/// # Returns
///
/// A result containing the welcome screen as a JSON value.
#[allow(dead_code)]
pub async fn get_guild_welcome_screen(client: &Client, token: &str, guild_id: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/welcome-screen", guild_id);
    let response: Value = client.get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

/// Modifies the welcome screen of a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild to modify the welcome screen for.
/// * `settings` - The JSON value of the settings to update.
///
/// # Returns
///
/// A result indicating success or failure.
#[allow(dead_code)]
pub async fn modify_guild_welcome_screen(client: &Client, token: &str, guild_id: &str, settings: Value) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/welcome-screen", guild_id);
    
    client.patch(&url)
        .bearer_auth(token)
        .json(&settings)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}

/// Fetches the onboarding settings of a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild to fetch the onboarding settings for.
///
/// # Returns
///
/// A result containing the onboarding settings as a JSON value.
#[allow(dead_code)]
pub async fn get_guild_onboarding(client: &Client, token: &str, guild_id: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/onboarding", guild_id);
    let response: Value = client.get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

/// Modifies the onboarding settings of a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild to modify the onboarding settings for.
/// * `settings` - The JSON value of the settings to update.
///
/// # Returns
///
/// A result indicating success or failure.
#[allow(dead_code)]
pub async fn modify_guild_onboarding(client: &Client, token: &str, guild_id: &str, settings: Value) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/onboarding", guild_id);
    
    client.put(&url)
        .bearer_auth(token)
        .json(&settings)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}

/// Modifies the current user's voice state in a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild.
/// * `settings` - The JSON value of the voice state settings.
///
/// # Returns
///
/// A result indicating success or failure.
#[allow(dead_code)]
pub async fn modify_current_user_voice_state(client: &Client, token: &str, guild_id: &str, settings: Value) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/voice-states/@me", guild_id);
    
    client.patch(&url)
        .bearer_auth(token)
        .json(&settings)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}

/// Modifies a user's voice state in a Discord guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild.
/// * `user_id` - The ID of the user.
/// * `settings` - The JSON value of the voice state settings.
///
/// # Returns
///
/// A result indicating success or failure.
#[allow(dead_code)]
pub async fn modify_user_voice_state(client: &Client, token: &str, guild_id: &str, user_id: &str, settings: Value) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/voice-states/{}", guild_id, user_id);
    
    client.patch(&url)
        .bearer_auth(token)
        .json(&settings)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}
