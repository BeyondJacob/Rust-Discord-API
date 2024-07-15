use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use async_trait::async_trait;
use reqwest::Client;
use std::error::Error;

#[async_trait]
/// The `Command` trait defines a common interface for all commands.
/// Each command must implement the `execute` method which handles the command's logic.
pub trait Command: Send + Sync {
    /// Execute the command.
    ///
    /// # Arguments
    ///
    /// * `client` - The HTTP client used to send requests.
    /// * `token` - The bot token for authentication.
    /// * `channel_id` - The ID of the channel where the command was invoked.
    /// * `args` - The arguments passed to the command.
    ///
    /// # Returns
    ///
    /// A result indicating success or failure.
    async fn execute(&self, client: &Client, token: &str, channel_id: &str, args: &str) -> Result<(), Box<dyn Error>>;
}

/// The `CommandRouter` struct is responsible for managing and dispatching commands.
pub struct CommandRouter {
    commands: HashMap<String, Arc<dyn Command>>,
}

impl CommandRouter {
    /// Create a new `CommandRouter`.
    ///
    /// # Examples
    ///
    /// ```
    /// use exampleapp::CommandRouter;
    ///
    /// let command_router = CommandRouter::new();
    /// ```
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }

    /// Register a command with the router.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the command (e.g., "!ping").
    /// * `command` - The command to register.
    ///
    /// # Examples
    ///
    /// ```
    /// use exampleapp::{CommandRouter, Command};
    /// use std::sync::Arc;
    /// use async_trait::async_trait;
    /// use reqwest::Client;
    /// use std::error::Error;
    ///
    /// struct PingCommand;
    ///
    /// #[async_trait]
    /// impl Command for PingCommand {
    ///     async fn execute(&self, client: &Client, token: &str, channel_id: &str, args: &str) -> Result<(), Box<dyn Error>> {
    ///         println!("Pong!");
    ///         Ok(())
    ///     }
    /// }
    ///
    /// let mut command_router = CommandRouter::new();
    /// command_router.register_command("!ping", Arc::new(PingCommand));
    /// ```
    pub fn register_command(&mut self, name: &str, command: Arc<dyn Command>) {
        self.commands.insert(name.to_string(), command);
    }

    /// Dispatch a command based on the input content.
    ///
    /// # Arguments
    ///
    /// * `client` - The HTTP client used to send requests.
    /// * `token` - The bot token for authentication.
    /// * `channel_id` - The ID of the channel where the command was invoked.
    /// * `content` - The content of the message.
    ///
    /// # Examples
    ///
    /// ```
    /// use exampleapp::{CommandRouter, Command};
    /// use std::sync::Arc;
    /// use tokio::sync::RwLock;
    /// use reqwest::Client;
    /// use std::env;
    /// use async_trait::async_trait;
    /// use std::error::Error;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    ///
    ///     let client = Client::new();
    ///     let mut command_router = CommandRouter::new();
    ///
    ///     struct PingCommand;
    ///
    ///     #[async_trait]
    ///     impl Command for PingCommand {
    ///         async fn execute(&self, client: &Client, token: &str, channel_id: &str, args: &str) -> Result<(), Box<dyn Error>> {
    ///             println!("Pong!");
    ///             Ok(())
    ///         }
    ///     }
    ///
    ///     command_router.register_command("!ping", Arc::new(PingCommand));
    ///
    ///     let command_router = Arc::new(RwLock::new(command_router));
    ///
    ///     let simulated_messages = vec![
    ///         ("!ping", "channel_id_1"),
    ///     ];
    ///
    ///     for (content, channel_id) in simulated_messages {
    ///         let router = command_router.read().await;
    ///         router.dispatch(&client, &token, channel_id, content).await?;
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn dispatch(&self, client: &Client, token: &str, channel_id: &str, content: &str) -> Result<(), Box<dyn Error>> {
        let parts: Vec<&str> = content.splitn(2, ' ').collect();
        let command_name = parts[0];
        let args = if parts.len() > 1 { parts[1] } else { "" };

        if let Some(command) = self.commands.get(command_name) {
            command.execute(client, token, channel_id, args).await?;
        } else {
            println!("Command not found: {}", command_name);
        }

        Ok(())
    }
}
