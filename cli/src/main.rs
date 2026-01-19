//! Instagram DM CLI
//!
//! A command-line interface for Instagram Direct Messages.
//! Communicates with a local Python/FastAPI server that handles Instagram API.

mod client;
mod commands;
mod crypto;
mod models;

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;

use client::ApiClient;

/// Instagram DM CLI - Manage your Instagram DMs from the terminal
#[derive(Parser)]
#[command(name = "insta")]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Server URL (default: http://localhost:8000)
    #[arg(short, long, global = true)]
    server: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Login to Instagram (interactive prompts for credentials)
    Login {
        /// Username (optional - will prompt if not provided)
        #[arg(short, long)]
        username: Option<String>,

        /// Password (optional - will prompt securely if not provided)
        #[arg(short, long)]
        password: Option<String>,
    },

    /// Logout from Instagram
    Logout,

    /// Check server status and authentication
    Status,

    /// Show current logged-in user info
    Me,

    /// Show inbox (list of conversations)
    Inbox {
        /// Number of threads to show (default: 20)
        #[arg(short, long, default_value = "20")]
        limit: u32,

        /// Show only unread conversations
        #[arg(short = 'u', long)]
        unread: bool,
    },

    /// Open chat by inbox number (eg: ig open 1)
    Open {
        /// Conversation number from inbox (1, 2, 3...)
        number: usize,
    },

    /// Search for a user
    Search {
        /// Username to search for
        query: String,
    },

    /// Show messages in a thread (by ID or @username)
    Thread {
        /// Thread ID or @username
        target: String,

        /// Number of messages to show (default: 20)
        #[arg(short, long, default_value = "20")]
        limit: u32,
    },

    /// Send a message to a user by username
    Send {
        /// Username to send to (without @)
        username: String,

        /// Message text (optional - will prompt if not provided)
        #[arg(short, long)]
        message: Option<String>,
    },

    /// Reply to a thread
    Reply {
        /// Thread ID
        thread_id: String,

        /// Message text (optional - will prompt if not provided)
        #[arg(short, long)]
        message: Option<String>,
    },

    /// Start interactive chat with a user
    Chat {
        /// Username to chat with (without @)
        username: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let client = ApiClient::new(cli.server.as_deref());

    match cli.command {
        Commands::Login { username, password } => {
            if let (Some(u), Some(p)) = (username.as_ref(), password.as_ref()) {
                // Non-interactive mode with provided credentials
                commands::login_with_credentials(&client, u, p).await
            } else if let Some(u) = username.as_ref() {
                // Username provided, prompt for password only
                use dialoguer::Password;
                println!("{}", "Instagram Login".bold().cyan());
                println!("{}", "━".repeat(40).dimmed());
                println!(
                    "{}",
                    "Your password will be encrypted before transmission.".dimmed()
                );
                println!();

                let password: String = Password::new()
                    .with_prompt("Password")
                    .interact()?;

                commands::login_with_credentials(&client, u, &password).await
            } else {
                // Full interactive mode
                commands::login_interactive(&client).await
            }
        }

        Commands::Logout => commands::logout(&client).await,

        Commands::Status => commands::status(&client).await,

        Commands::Me => commands::show_me(&client).await,

        Commands::Inbox { limit, unread } => commands::show_inbox(&client, limit, unread).await,

        Commands::Open { number } => commands::open_by_number(&client, number).await,

        Commands::Search { query } => commands::search_user(&client, &query).await,

        Commands::Thread { target, limit } => {
            commands::show_thread_or_user(&client, &target, limit).await
        }

        Commands::Send { username, message } => {
            commands::send_to_user(&client, &username, message.as_deref()).await
        }

        Commands::Reply { thread_id, message } => {
            commands::send_to_thread(&client, &thread_id, message.as_deref()).await
        }

        Commands::Chat { username } => commands::chat_with_user(&client, &username).await,
    }
}
