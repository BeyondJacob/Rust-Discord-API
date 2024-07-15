use reqwest::Client;
use serde_json::Value;
use std::error::Error;

/// Fetches information about a Discord channel.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `channel_id` - The ID of the channel to fetch information for.
///
/// # Returns
///
/// A result containing the channel information as a JSON value.
#[allow(dead_code)]
pub async fn fetch_channel_info(client: &Client, token: &str, channel_id: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/channels/{}", channel_id);
    let response: Value = client.get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

/// Modifies a Discord channel.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `channel_id` - The ID of the channel to modify.
/// * `settings` - The JSON value of the settings to update.
///
/// # Returns
///
/// A result indicating success or failure.
#[allow(dead_code)]
pub async fn modify_channel(client: &Client, token: &str, channel_id: &str, settings: Value) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/channels/{}", channel_id);
    
    client.patch(&url)
        .bearer_auth(token)
        .json(&settings)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}

/// Deletes a Discord channel.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `channel_id` - The ID of the channel to delete.
///
/// # Returns
///
/// A result indicating success or failure.
#[allow(dead_code)]
pub async fn delete_channel(client: &Client, token: &str, channel_id: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/channels/{}", channel_id);
    
    client.delete(&url)
        .bearer_auth(token)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}

/// Fetches messages from a Discord channel.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `channel_id` - The ID of the channel to fetch messages from.
///
/// # Returns
///
/// A result containing the channel messages as a JSON value.
#[allow(dead_code)]
pub async fn get_channel_messages(client: &Client, token: &str, channel_id: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/channels/{}/messages", channel_id);
    let response: Value = client.get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

/// Fetches a single message from a Discord channel.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `channel_id` - The ID of the channel where the message is located.
/// * `message_id` - The ID of the message to fetch.
///
/// # Returns
///
/// A result containing the message information as a JSON value.
#[allow(dead_code)]
pub async fn get_channel_message(client: &Client, token: &str, channel_id: &str, message_id: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/channels/{}/messages/{}", channel_id, message_id);
    let response: Value = client.get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

/// Crossposts a message in an Announcement Channel.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `channel_id` - The ID of the announcement channel.
/// * `message_id` - The ID of the message to crosspost.
///
/// # Returns
///
/// A result indicating success or failure.
#[allow(dead_code)]
pub async fn crosspost_message(client: &Client, token: &str, channel_id: &str, message_id: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/channels/{}/messages/{}/crosspost", channel_id, message_id);
    
    client.post(&url)
        .bearer_auth(token)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}

/// Creates a reaction for a message.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `channel_id` - The ID of the channel where the message is located.
/// * `message_id` - The ID of the message to react to.
/// * `emoji` - The emoji to react with.
///
/// # Returns
///
/// A result indicating success or failure.
#[allow(dead_code)]
pub async fn create_reaction(client: &Client, token: &str, channel_id: &str, message_id: &str, emoji: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/channels/{}/messages/{}/reactions/{}/@me", channel_id, message_id, emoji);
    
    client.put(&url)
        .bearer_auth(token)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}

/// Deletes the bot's reaction to a message.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `channel_id` - The ID of the channel where the message is located.
/// * `message_id` - The ID of the message to remove the reaction from.
/// * `emoji` - The emoji to remove.
///
/// # Returns
///
/// A result indicating success or failure.
#[allow(dead_code)]
pub async fn delete_own_reaction(client: &Client, token: &str, channel_id: &str, message_id: &str, emoji: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/channels/{}/messages/{}/reactions/{}/@me", channel_id, message_id, emoji);
    
    client.delete(&url)
        .bearer_auth(token)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}

/// Deletes a user's reaction to a message.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `channel_id` - The ID of the channel where the message is located.
/// * `message_id` - The ID of the message to remove the reaction from.
/// * `emoji` - The emoji to remove.
/// * `user_id` - The ID of the user whose reaction to remove.
///
/// # Returns
///
/// A result indicating success or failure.
#[allow(dead_code)]
pub async fn delete_user_reaction(client: &Client, token: &str, channel_id: &str, message_id: &str, emoji: &str, user_id: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/channels/{}/messages/{}/reactions/{}/{}", channel_id, message_id, emoji, user_id);
    
    client.delete(&url)
        .bearer_auth(token)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}

/// Gets all reactions for a message.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `channel_id` - The ID of the channel where the message is located.
/// * `message_id` - The ID of the message to get reactions for.
///
/// # Returns
///
/// A result containing the reactions as a JSON value.
#[allow(dead_code)]
pub async fn get_reactions(client: &Client, token: &str, channel_id: &str, message_id: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/channels/{}/messages/{}/reactions", channel_id, message_id);
    let response: Value = client.get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

/// Deletes all reactions for a message.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `channel_id` - The ID of the channel where the message is located.
/// * `message_id` - The ID of the message to delete reactions from.
///
/// # Returns
///
/// A result indicating success or failure.
#[allow(dead_code)]
pub async fn delete_all_reactions(client: &Client, token: &str, channel_id: &str, message_id: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/channels/{}/messages/{}/reactions", channel_id, message_id);
    
    client.delete(&url)
        .bearer_auth(token)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}

/// Deletes all reactions for a message with a specific emoji.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `channel_id` - The ID of the channel where the message is located.
/// * `message_id` - The ID of the message to delete reactions from.
/// * `emoji` - The emoji to remove reactions for.
///
/// # Returns
///
/// A result indicating success or failure.
#[allow(dead_code)]
pub async fn delete_all_reactions_for_emoji(client: &Client, token: &str, channel_id: &str, message_id: &str, emoji: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/channels/{}/messages/{}/reactions/{}", channel_id, message_id, emoji);
    
    client.delete(&url)
        .bearer_auth(token)
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
    let body = serde_json::json!({ "content": new_content });
    
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

/// Bulk deletes messages in a specified Discord channel.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `channel_id` - The ID of the channel where the messages are located.
/// * `message_ids` - A list of message IDs to delete.
///
/// # Returns
///
/// A result indicating success or failure.
#[allow(dead_code)]
pub async fn bulk_delete_messages(client: &Client, token: &str, channel_id: &str, message_ids: Vec<&str>) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/channels/{}/messages/bulk-delete", channel_id);
    let body = serde_json::json!({ "messages": message_ids });
    
    client.post(&url)
        .bearer_auth(token)
        .json(&body)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}

/// Edits channel permissions.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `channel_id` - The ID of the channel to edit permissions for.
/// * `overwrite_id` - The ID of the overwrite to edit.
/// * `permissions` - The JSON value of the permissions to update.
///
/// # Returns
///
/// A result indicating success or failure.
#[allow(dead_code)]
pub async fn edit_channel_permissions(client: &Client, token: &str, channel_id: &str, overwrite_id: &str, permissions: Value) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/channels/{}/permissions/{}", channel_id, overwrite_id);
    
    client.put(&url)
        .bearer_auth(token)
        .json(&permissions)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}

/// Fetches channel invites.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `channel_id` - The ID of the channel to fetch invites for.
///
/// # Returns
///
/// A result containing the channel invites as a JSON value.
#[allow(dead_code)]
pub async fn get_channel_invites(client: &Client, token: &str, channel_id: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/channels/{}/invites", channel_id);
    let response: Value = client.get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

/// Creates a channel invite.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `channel_id` - The ID of the channel to create an invite for.
/// * `invite_settings` - The JSON value of the invite settings.
///
/// # Returns
///
/// A result containing the created invite as a JSON value.
#[allow(dead_code)]
pub async fn create_channel_invite(client: &Client, token: &str, channel_id: &str, invite_settings: Value) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/channels/{}/invites", channel_id);
    let response: Value = client.post(&url)
        .bearer_auth(token)
        .json(&invite_settings)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

/// Deletes a channel permission overwrite.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `channel_id` - The ID of the channel to delete the permission overwrite for.
/// * `overwrite_id` - The ID of the overwrite to delete.
///
/// # Returns
///
/// A result indicating success or failure.
#[allow(dead_code)]
pub async fn delete_channel_permission(client: &Client, token: &str, channel_id: &str, overwrite_id: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/channels/{}/permissions/{}", channel_id, overwrite_id);
    
    client.delete(&url)
        .bearer_auth(token)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}

/// Follows an announcement channel.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `channel_id` - The ID of the announcement channel to follow.
/// * `webhook_channel_id` - The ID of the channel to send messages to.
///
/// # Returns
///
/// A result containing the follow response as a JSON value.
#[allow(dead_code)]
pub async fn follow_announcement_channel(client: &Client, token: &str, channel_id: &str, webhook_channel_id: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/channels/{}/followers", channel_id);
    let body = serde_json::json!({ "webhook_channel_id": webhook_channel_id });
    let response: Value = client.post(&url)
        .bearer_auth(token)
        .json(&body)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

/// Triggers a typing indicator in a specified Discord channel.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `channel_id` - The ID of the channel to trigger the typing indicator in.
///
/// # Returns
///
/// A result indicating success or failure.
#[allow(dead_code)]
pub async fn trigger_typing_indicator(client: &Client, token: &str, channel_id: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/channels/{}/typing", channel_id);
    
    client.post(&url)
        .bearer_auth(token)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}

/// Fetches pinned messages from a Discord channel.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `channel_id` - The ID of the channel to fetch pinned messages from.
///
/// # Returns
///
/// A result containing the pinned messages as a JSON value.
#[allow(dead_code)]
pub async fn get_pinned_messages(client: &Client, token: &str, channel_id: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/channels/{}/pins", channel_id);
    let response: Value = client.get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
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

/// Adds a recipient to a Group DM.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `channel_id` - The ID of the Group DM.
/// * `user_id` - The ID of the user to add.
/// * `access_token` - The OAuth2 access token of the user.
/// * `nick` - The nickname of the user.
///
/// # Returns
///
/// A result indicating success or failure.
#[allow(dead_code)]
pub async fn group_dm_add_recipient(client: &Client, token: &str, channel_id: &str, user_id: &str, access_token: &str, nick: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/channels/{}/recipients/{}", channel_id, user_id);
    let body = serde_json::json!({ "access_token": access_token, "nick": nick });
    
    client.put(&url)
        .bearer_auth(token)
        .json(&body)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}

/// Removes a recipient from a Group DM.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `channel_id` - The ID of the Group DM.
/// * `user_id` - The ID of the user to remove.
///
/// # Returns
///
/// A result indicating success or failure.
#[allow(dead_code)]
pub async fn group_dm_remove_recipient(client: &Client, token: &str, channel_id: &str, user_id: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/channels/{}/recipients/{}", channel_id, user_id);
    
    client.delete(&url)
        .bearer_auth(token)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}

/// Starts a thread from an existing message.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `channel_id` - The ID of the channel where the message is located.
/// * `message_id` - The ID of the message to start the thread from.
/// * `thread_settings` - The JSON value of the thread settings.
///
/// # Returns
///
/// A result containing the created thread information as a JSON value.
#[allow(dead_code)]
pub async fn start_thread_from_message(client: &Client, token: &str, channel_id: &str, message_id: &str, thread_settings: Value) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/channels/{}/messages/{}/threads", channel_id, message_id);
    let response: Value = client.post(&url)
        .bearer_auth(token)
        .json(&thread_settings)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

/// Starts a thread without an existing message.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `channel_id` - The ID of the channel to start the thread in.
/// * `thread_settings` - The JSON value of the thread settings.
///
/// # Returns
///
/// A result containing the created thread information as a JSON value.
#[allow(dead_code)]
pub async fn start_thread_without_message(client: &Client, token: &str, channel_id: &str, thread_settings: Value) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/channels/{}/threads", channel_id);
    let response: Value = client.post(&url)
        .bearer_auth(token)
        .json(&thread_settings)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

/// Joins a thread.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `channel_id` - The ID of the thread to join.
///
/// # Returns
///
/// A result indicating success or failure.
#[allow(dead_code)]
pub async fn join_thread(client: &Client, token: &str, channel_id: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/channels/{}/thread-members/@me", channel_id);
    
    client.put(&url)
        .bearer_auth(token)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}

/// Adds a member to a thread.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `channel_id` - The ID of the thread.
/// * `user_id` - The ID of the user to add.
///
/// # Returns
///
/// A result indicating success or failure.
#[allow(dead_code)]
pub async fn add_thread_member(client: &Client, token: &str, channel_id: &str, user_id: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/channels/{}/thread-members/{}", channel_id, user_id);
    
    client.put(&url)
        .bearer_auth(token)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}

/// Removes a member from a thread.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `channel_id` - The ID of the thread.
/// * `user_id` - The ID of the user to remove.
///
/// # Returns
///
/// A result indicating success or failure.
#[allow(dead_code)]
pub async fn remove_thread_member(client: &Client, token: &str, channel_id: &str, user_id: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/channels/{}/thread-members/{}", channel_id, user_id);
    
    client.delete(&url)
        .bearer_auth(token)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}

/// Gets information about a thread member.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `channel_id` - The ID of the thread.
/// * `user_id` - The ID of the user to get information for.
///
/// # Returns
///
/// A result containing the thread member information as a JSON value.
#[allow(dead_code)]
pub async fn get_thread_member(client: &Client, token: &str, channel_id: &str, user_id: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/channels/{}/thread-members/{}", channel_id, user_id);
    let response: Value = client.get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

/// Lists members of a thread.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `channel_id` - The ID of the thread.
///
/// # Returns
///
/// A result containing the list of thread members as a JSON value.
#[allow(dead_code)]
pub async fn list_thread_members(client: &Client, token: &str, channel_id: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/channels/{}/thread-members", channel_id);
    let response: Value = client.get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

/// Lists public archived threads in a channel.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `channel_id` - The ID of the channel.
///
/// # Returns
///
/// A result containing the list of public archived threads as a JSON value.
#[allow(dead_code)]
pub async fn list_public_archived_threads(client: &Client, token: &str, channel_id: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/channels/{}/threads/archived/public", channel_id);
    let response: Value = client.get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

/// Lists private archived threads in a channel.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `channel_id` - The ID of the channel.
///
/// # Returns
///
/// A result containing the list of private archived threads as a JSON value.
#[allow(dead_code)]
pub async fn list_private_archived_threads(client: &Client, token: &str, channel_id: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/channels/{}/threads/archived/private", channel_id);
    let response: Value = client.get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

/// Lists joined private archived threads.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `channel_id` - The ID of the channel.
///
/// # Returns
///
/// A result containing the list of joined private archived threads as a JSON value.
#[allow(dead_code)]
pub async fn list_joined_private_archived_threads(client: &Client, token: &str, channel_id: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/channels/{}/users/@me/threads/archived/private", channel_id);
    let response: Value = client.get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}
