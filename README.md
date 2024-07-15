# Rust Discord API

A Discord bot framework written in Rust. It provides a structured and easy way to create and manage commands for your Discord bot.

## Features

- Easily register and manage commands.
- Asynchronous command execution using `tokio`.
- Supports commands organized in subdirectories.

## Installation

Add `Rust-Discord-API` to your `Cargo.toml`:

```toml
[dependencies]
rust-discord-api = "0.1.0"
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.12.5", features = ["json"] }
async-trait = "0.1"
```

## Usage
Define a Command
To define a command, implement the Command trait:

```rust
use async_trait::async_trait;
use reqwest::Client;
use exampleapp::Command;
use std::error::Error;

pub struct PingCommand;

#[async_trait]
impl Command for PingCommand {
    async fn execute(&self, client: &Client, token: &str, channel_id: &str, _args: &str) -> Result<(), Box<dyn Error>> {
        // Implement your message sending logic here
        println!("Pong!");
        Ok(())
    }
}
```

## Register Commands
Register your commands with the CommandRouter:

```rust
use exampleapp::{CommandRouter, Command};
use std::sync::Arc;
use tokio::sync::RwLock;
use reqwest::Client;
use std::env;
use async_trait::async_trait;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let client = Client::new();
    let mut command_router = CommandRouter::new();

    command_router.register_command("!ping", Arc::new(PingCommand));

    let command_router = Arc::new(RwLock::new(command_router));

    // Simulated message handling loop
    let simulated_messages = vec![
        ("!ping", "channel_id_1"),
    ];

    for (content, channel_id) in simulated_messages {
        let router = command_router.read().await;
        router.dispatch(&client, &token, channel_id, content).await?;
    }

    Ok(())
}
```

## Organize Commands in Subdirectories
Commands can be organized in subdirectories for better structure. For example:

```css
src/
├── main.rs
├── commands/
│   ├── mod.rs
│   ├── ping.rs
│   ├── admin/
│   │   ├── kick.rs
│   │   ├── ban.rs
```

## Example src/commands/mod.rs

```rust
pub mod ping;
pub mod admin {
    pub mod kick;
    pub mod ban;
}
```

## Example src/commands/ping.rs

```rust
use async_trait::async_trait;
use reqwest::Client;
use exampleapp::Command;
use std::error::Error;

pub struct PingCommand;

#[async_trait]
impl Command for PingCommand {
    async fn execute(&self, client: &Client, token: &str, channel_id: &str, _args: &str) -> Result<(), Box<dyn Error>> {
        println!("Pong!");
        Ok(())
    }
}
```

## License
This project is licensed under the MIT License. See the LICENSE file for details.