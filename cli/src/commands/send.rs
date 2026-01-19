//! Send message commands

use anyhow::Result;
use dialoguer::Input;

use crate::client::ApiClient;
use crate::colors::Theme;

/// Send a message to a user (interactive or with provided message)
pub async fn send_to_user(client: &ApiClient, username: &str, message: Option<&str>) -> Result<()> {
    let text = match message {
        Some(m) => m.to_string(),
        None => {
            // Interactive mode
            Input::new()
                .with_prompt(format!("Message to @{}", username))
                .interact_text()?
        }
    };

    if text.trim().is_empty() {
        println!("{}", Theme::warning("Message cannot be empty."));
        return Ok(());
    }

    println!("{}", Theme::muted(&format!("Sending to @{}...", username)));

    match client.send_to_user(username, &text).await {
        Ok(response) => {
            if response.success {
                println!(
                    "{} {}",
                    Theme::check(),
                    Theme::success(&format!("Message sent to @{}", username))
                );
            } else {
                println!(
                    "{} {}",
                    Theme::cross(),
                    Theme::error(&response.error.unwrap_or("Failed to send message".to_string()))
                );
            }
            Ok(())
        }
        Err(e) => {
            println!("{} {}", Theme::cross(), Theme::error(&format!("{}", e)));
            Err(e)
        }
    }
}

/// Send a message to an existing thread (interactive or with provided message)
pub async fn send_to_thread(
    client: &ApiClient,
    thread_id: &str,
    message: Option<&str>,
) -> Result<()> {
    let text = match message {
        Some(m) => m.to_string(),
        None => {
            // Interactive mode
            Input::new()
                .with_prompt("Message")
                .interact_text()?
        }
    };

    if text.trim().is_empty() {
        println!("{}", Theme::warning("Message cannot be empty."));
        return Ok(());
    }

    println!("{}", Theme::muted("Sending message..."));

    match client.send_to_thread(thread_id, &text).await {
        Ok(response) => {
            if response.success {
                println!("{} {}", Theme::check(), Theme::success("Message sent!"));
            } else {
                println!(
                    "{} {}",
                    Theme::cross(),
                    Theme::error(&response.error.unwrap_or("Failed to send message".to_string()))
                );
            }
            Ok(())
        }
        Err(e) => {
            println!("{} {}", Theme::cross(), Theme::error(&format!("{}", e)));
            Err(e)
        }
    }
}

/// Interactive chat with a user by username
pub async fn chat_with_user(client: &ApiClient, username: &str) -> Result<()> {
    println!("{} {}", Theme::header("Chat with"), Theme::username(&format!("@{}", username)));
    println!(
        "{}",
        Theme::muted("Type your messages. Empty line to exit.")
    );
    println!();

    loop {
        let text: String = Input::new()
            .with_prompt(">")
            .allow_empty(true)
            .interact_text()?;

        if text.trim().is_empty() {
            println!("{}", Theme::muted("Exiting chat mode."));
            break;
        }

        match client.send_to_user(username, &text).await {
            Ok(response) => {
                if response.success {
                    println!("{} {}", Theme::check(), Theme::muted("Sent"));
                } else {
                    println!(
                        "{} {}",
                        Theme::cross(),
                        Theme::error(&response.error.unwrap_or("Failed".to_string()))
                    );
                }
            }
            Err(e) => {
                println!("{} {}", Theme::cross(), Theme::error(&format!("{}", e)));
            }
        }
    }

    Ok(())
}
