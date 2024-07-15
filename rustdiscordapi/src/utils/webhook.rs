use reqwest::Client;
use serde_json::Value;
use std::error::Error;

#[allow(dead_code)]
/// Creates a new webhook.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `channel_id` - The ID of the channel to create the webhook in.
/// * `webhook_settings` - The JSON value of the webhook settings.
///
/// # Returns
///
/// A result containing the created webhook information as a JSON value.
pub async fn create_webhook(client: &Client, token: &str, channel_id: &str, webhook_settings: Value) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/channels/{}/webhooks", channel_id);
    let response: Value = client.post(&url)
        .bearer_auth(token)
        .json(&webhook_settings)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

#[allow(dead_code)]
/// Fetches webhooks for a specific channel.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `channel_id` - The ID of the channel to fetch webhooks for.
///
/// # Returns
///
/// A result containing the list of webhooks as a JSON value.
pub async fn get_channel_webhooks(client: &Client, token: &str, channel_id: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/channels/{}/webhooks", channel_id);
    let response: Value = client.get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

#[allow(dead_code)]
/// Fetches webhooks for a specific guild.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `guild_id` - The ID of the guild to fetch webhooks for.
///
/// # Returns
///
/// A result containing the list of webhooks as a JSON value.
pub async fn get_guild_webhooks(client: &Client, token: &str, guild_id: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/guilds/{}/webhooks", guild_id);
    let response: Value = client.get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

#[allow(dead_code)]
/// Fetches a webhook by its ID.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `webhook_id` - The ID of the webhook to fetch.
///
/// # Returns
///
/// A result containing the webhook information as a JSON value.
pub async fn get_webhook(client: &Client, token: &str, webhook_id: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/webhooks/{}", webhook_id);
    let response: Value = client.get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

#[allow(dead_code)]
/// Fetches a webhook by its ID and token.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `webhook_id` - The ID of the webhook to fetch.
/// * `webhook_token` - The token of the webhook to fetch.
///
/// # Returns
///
/// A result containing the webhook information as a JSON value.
pub async fn get_webhook_with_token(client: &Client, webhook_id: &str, webhook_token: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/webhooks/{}/{}", webhook_id, webhook_token);
    let response: Value = client.get(&url)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

#[allow(dead_code)]
/// Modifies a webhook.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `webhook_id` - The ID of the webhook to modify.
/// * `settings` - The JSON value of the settings to update.
///
/// # Returns
///
/// A result containing the modified webhook information as a JSON value.
pub async fn modify_webhook(client: &Client, token: &str, webhook_id: &str, settings: Value) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/webhooks/{}", webhook_id);
    let response: Value = client.patch(&url)
        .bearer_auth(token)
        .json(&settings)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

#[allow(dead_code)]
/// Modifies a webhook using its token.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `webhook_id` - The ID of the webhook to modify.
/// * `webhook_token` - The token of the webhook to modify.
/// * `settings` - The JSON value of the settings to update.
///
/// # Returns
///
/// A result containing the modified webhook information as a JSON value.
pub async fn modify_webhook_with_token(client: &Client, webhook_id: &str, webhook_token: &str, settings: Value) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/webhooks/{}/{}", webhook_id, webhook_token);
    let response: Value = client.patch(&url)
        .json(&settings)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

#[allow(dead_code)]
/// Deletes a webhook.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `token` - The bot token for authentication.
/// * `webhook_id` - The ID of the webhook to delete.
///
/// # Returns
///
/// A result indicating success or failure.
pub async fn delete_webhook(client: &Client, token: &str, webhook_id: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/webhooks/{}", webhook_id);
    
    client.delete(&url)
        .bearer_auth(token)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}

#[allow(dead_code)]
/// Deletes a webhook using its token.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `webhook_id` - The ID of the webhook to delete.
/// * `webhook_token` - The token of the webhook to delete.
///
/// # Returns
///
/// A result indicating success or failure.
pub async fn delete_webhook_with_token(client: &Client, webhook_id: &str, webhook_token: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/webhooks/{}/{}", webhook_id, webhook_token);
    
    client.delete(&url)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}

#[allow(dead_code)]
/// Executes a webhook.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `webhook_id` - The ID of the webhook to execute.
/// * `webhook_token` - The token of the webhook to execute.
/// * `payload` - The JSON payload to send.
///
/// # Returns
///
/// A result indicating success or failure.
pub async fn execute_webhook(client: &Client, webhook_id: &str, webhook_token: &str, payload: Value) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/webhooks/{}/{}", webhook_id, webhook_token);
    
    client.post(&url)
        .json(&payload)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}

#[allow(dead_code)]
/// Executes a Slack-compatible webhook.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `webhook_id` - The ID of the webhook to execute.
/// * `webhook_token` - The token of the webhook to execute.
/// * `payload` - The JSON payload to send.
///
/// # Returns
///
/// A result indicating success or failure.
pub async fn execute_slack_compatible_webhook(client: &Client, webhook_id: &str, webhook_token: &str, payload: Value) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/webhooks/{}/{}/slack", webhook_id, webhook_token);
    
    client.post(&url)
        .json(&payload)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}

#[allow(dead_code)]
/// Executes a GitHub-compatible webhook.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `webhook_id` - The ID of the webhook to execute.
/// * `webhook_token` - The token of the webhook to execute.
/// * `payload` - The JSON payload to send.
///
/// # Returns
///
/// A result indicating success or failure.
pub async fn execute_github_compatible_webhook(client: &Client, webhook_id: &str, webhook_token: &str, payload: Value) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/webhooks/{}/{}/github", webhook_id, webhook_token);
    
    client.post(&url)
        .json(&payload)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}

#[allow(dead_code)]
/// Fetches a specific message from a webhook.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `webhook_id` - The ID of the webhook.
/// * `webhook_token` - The token of the webhook.
/// * `message_id` - The ID of the message to fetch.
///
/// # Returns
///
/// A result containing the message information as a JSON value.
pub async fn get_webhook_message(client: &Client, webhook_id: &str, webhook_token: &str, message_id: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/webhooks/{}/{}/messages/{}", webhook_id, webhook_token, message_id);
    let response: Value = client.get(&url)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

#[allow(dead_code)]
/// Edits a specific message from a webhook.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `webhook_id` - The ID of the webhook.
/// * `webhook_token` - The token of the webhook.
/// * `message_id` - The ID of the message to edit.
/// * `new_content` - The new content of the message.
///
/// # Returns
///
/// A result containing the edited message information as a JSON value.
pub async fn edit_webhook_message(client: &Client, webhook_id: &str, webhook_token: &str, message_id: &str, new_content: Value) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/webhooks/{}/{}/messages/{}", webhook_id, webhook_token, message_id);
    let response: Value = client.patch(&url)
        .json(&new_content)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}

#[allow(dead_code)]
/// Deletes a specific message from a webhook.
///
/// # Arguments
///
/// * `client` - The HTTP client used to send the request.
/// * `webhook_id` - The ID of the webhook.
/// * `webhook_token` - The token of the webhook.
/// * `message_id` - The ID of the message to delete.
///
/// # Returns
///
/// A result indicating success or failure.
pub async fn delete_webhook_message(client: &Client, webhook_id: &str, webhook_token: &str, message_id: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("https://discord.com/api/v9/webhooks/{}/{}/messages/{}", webhook_id, webhook_token, message_id);
    
    client.delete(&url)
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}
