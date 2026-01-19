//! Authentication commands with interactive prompts

use anyhow::Result;
use colored::Colorize;
use dialoguer::{Input, Password};

use crate::client::ApiClient;

/// Interactive login with encrypted password
pub async fn login_interactive(client: &ApiClient) -> Result<()> {
    println!("{}", "Instagram Login".bold().cyan());
    println!("{}", "━".repeat(40).dimmed());
    println!(
        "{}",
        "Your password will be encrypted before transmission.".dimmed()
    );
    println!();

    // Prompt for username
    let username: String = Input::new()
        .with_prompt("Username")
        .interact_text()?;

    // Prompt for password (hidden input)
    let password: String = Password::new()
        .with_prompt("Password")
        .interact()?;

    println!();
    println!("{}", "Authenticating...".dimmed());

    // Attempt login with encrypted password
    match client.login(&username, &password).await {
        Ok(response) => {
            if response.success {
                println!("{} {}", "✓".green().bold(), "Login successful!".green());
                if let Some(user) = response.user {
                    println!(
                        "  {} {} ({})",
                        "Logged in as:".dimmed(),
                        user.username.bold(),
                        user.full_name.unwrap_or_default()
                    );
                }
            } else {
                println!(
                    "{} {}",
                    "✗".red().bold(),
                    response.message.unwrap_or("Login failed".to_string()).red()
                );
            }
            Ok(())
        }
        Err(e) => {
            println!("{} {}", "✗".red().bold(), format!("{}", e).red());
            Err(e)
        }
    }
}

/// Login with provided credentials (non-interactive)
pub async fn login_with_credentials(
    client: &ApiClient,
    username: &str,
    password: &str,
) -> Result<()> {
    println!("{}", "Authenticating...".dimmed());

    match client.login(username, password).await {
        Ok(response) => {
            if response.success {
                println!("{} {}", "✓".green().bold(), "Login successful!".green());
                if let Some(user) = response.user {
                    println!(
                        "  {} {} ({})",
                        "Logged in as:".dimmed(),
                        user.username.bold(),
                        user.full_name.unwrap_or_default()
                    );
                }
            } else {
                println!(
                    "{} {}",
                    "✗".red().bold(),
                    response.message.unwrap_or("Login failed".to_string()).red()
                );
            }
            Ok(())
        }
        Err(e) => {
            println!("{} {}", "✗".red().bold(), format!("{}", e).red());
            Err(e)
        }
    }
}

/// Logout from Instagram
pub async fn logout(client: &ApiClient) -> Result<()> {
    println!("{}", "Logging out...".dimmed());

    client.logout().await?;
    println!("{} {}", "✓".green().bold(), "Logged out successfully".green());
    Ok(())
}

/// Check authentication status
pub async fn status(client: &ApiClient) -> Result<()> {
    match client.health().await {
        Ok(health) => {
            println!("{}", "Server Status".bold().cyan());
            println!("{}", "━".repeat(40).dimmed());
            println!(
                "  {} {}",
                "Server:".dimmed(),
                health.status.green()
            );
            if health.authenticated {
                println!(
                    "  {} {} ({})",
                    "Status:".dimmed(),
                    "Authenticated".green(),
                    health.username.unwrap_or_default().bold()
                );
            } else {
                println!(
                    "  {} {}",
                    "Status:".dimmed(),
                    "Not authenticated".yellow()
                );
            }
            Ok(())
        }
        Err(e) => {
            println!(
                "{} {} {}",
                "✗".red().bold(),
                "Cannot connect to server:".red(),
                e
            );
            Err(e)
        }
    }
}

/// Show current logged-in user info
pub async fn show_me(client: &ApiClient) -> Result<()> {
    match client.health().await {
        Ok(health) => {
            if health.authenticated {
                println!();
                println!("{}", "Current User".bold().cyan());
                println!("{}", "━".repeat(40).dimmed());
                println!(
                    "  {} @{}",
                    "Username:".dimmed(),
                    health.username.unwrap_or_default().bold()
                );
                println!();
            } else {
                println!(
                    "{} {}",
                    "✗".yellow().bold(),
                    "Not logged in. Use 'ig login' first.".yellow()
                );
            }
            Ok(())
        }
        Err(e) => {
            println!(
                "{} {} {}",
                "✗".red().bold(),
                "Cannot connect to server:".red(),
                e
            );
            Err(e)
        }
    }
}

/// Search for a user by username
pub async fn search_user(client: &ApiClient, query: &str) -> Result<()> {
    // Remove @ prefix if present
    let username = query.trim_start_matches('@');

    println!("{}", format!("Searching for @{}...", username).dimmed());

    match client.search_user(username).await {
        Ok(response) => {
            if let Some(user) = response.user {
                println!();
                println!("{}", "User Found".bold().cyan());
                println!("{}", "━".repeat(40).dimmed());
                println!(
                    "  {} @{}",
                    "Username:".dimmed(),
                    user.username.bold()
                );
                if let Some(name) = user.full_name {
                    if !name.is_empty() {
                        println!("  {} {}", "Name:".dimmed(), name);
                    }
                }
                if let Some(verified) = user.is_verified {
                    if verified {
                        println!("  {} {}", "Verified:".dimmed(), "✓".blue());
                    }
                }
                if let Some(private) = user.is_private {
                    println!(
                        "  {} {}",
                        "Account:".dimmed(),
                        if private { "Private".yellow() } else { "Public".green() }
                    );
                }
                if let Some(followers) = user.follower_count {
                    println!("  {} {}", "Followers:".dimmed(), format_count(followers));
                }
                if let Some(following) = user.following_count {
                    println!("  {} {}", "Following:".dimmed(), format_count(following));
                }
                println!();
                println!(
                    "{}",
                    format!("Send message: ig send {} -m \"Hello!\"", user.username).dimmed()
                );
            } else {
                println!(
                    "{} {}",
                    "✗".yellow().bold(),
                    format!("User @{} not found", username).yellow()
                );
            }
            Ok(())
        }
        Err(e) => {
            println!("{} {}", "✗".red().bold(), format!("{}", e).red());
            Err(e)
        }
    }
}

/// Format large numbers (1000 -> 1K, 1000000 -> 1M)
fn format_count(n: u64) -> String {
    if n >= 1_000_000 {
        format!("{:.1}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.1}K", n as f64 / 1_000.0)
    } else {
        n.to_string()
    }
}
