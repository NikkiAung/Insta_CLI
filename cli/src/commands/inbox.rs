//! Inbox and thread commands

use anyhow::Result;

use crate::client::ApiClient;
use crate::colors::Theme;
use crate::models::Thread;
use crate::commands::chat_with_user;

/// Display inbox (list of conversations)
pub async fn show_inbox(client: &ApiClient, limit: u32, unread_only: bool) -> Result<()> {
    println!("{}", Theme::muted("Fetching inbox..."));

    let response = client.get_inbox(limit).await?;

    if !response.success {
        println!(
            "{} {}",
            Theme::cross(),
            Theme::error(&response.error.unwrap_or("Failed to fetch inbox".to_string()))
        );
        return Ok(());
    }

    let threads = response.threads.unwrap_or_default();

    // Filter to unread only if flag is set
    let threads: Vec<_> = if unread_only {
        threads.into_iter().filter(|t| t.has_unread.unwrap_or(false)).collect()
    } else {
        threads
    };

    if threads.is_empty() {
        if unread_only {
            println!("{}", Theme::muted("No unread conversations."));
        } else {
            println!("{}", Theme::muted("No conversations found."));
        }
        return Ok(());
    }

    println!();
    if unread_only {
        println!("{} {}", Theme::header("Inbox"), Theme::blue("(unread)"));
    } else {
        println!("{}", Theme::header("Inbox"));
    }
    println!("{}", Theme::separator(60));

    for (i, thread) in threads.iter().enumerate() {
        print_thread_summary(i + 1, thread);
    }

    println!("{}", Theme::separator(60));
    println!(
        "{}",
        Theme::muted(&format!("Showing {} conversations", threads.len()))
    );

    Ok(())
}

/// Display a specific thread with messages
pub async fn show_thread(client: &ApiClient, thread_id: &str, limit: u32) -> Result<()> {
    println!("{}", Theme::muted("Fetching messages..."));

    let response = client.get_thread(thread_id, limit).await?;

    if !response.success {
        println!(
            "{} {}",
            Theme::cross(),
            Theme::error(&response.error.unwrap_or("Failed to fetch thread".to_string()))
        );
        return Ok(());
    }

    let thread = match response.thread {
        Some(t) => t,
        None => {
            println!("{}", Theme::muted("Thread not found."));
            return Ok(());
        }
    };

    println!();
    let participants: Vec<&str> = thread.users.iter().map(|u| u.username.as_str()).collect();
    println!(
        "{} {}",
        Theme::header("Conversation with:"),
        Theme::username(&participants.join(", "))
    );
    println!("{}", Theme::separator(60));

    let messages = thread.messages.unwrap_or_default();

    if messages.is_empty() {
        println!("{}", Theme::muted("No messages in this thread."));
        return Ok(());
    }

    for msg in messages.iter().rev() {
        // Find the sender
        let sender = msg.user_id.as_ref().and_then(|uid| {
            thread.users.iter().find(|u| &u.pk == uid)
        }).map(|u| u.username.as_str()).unwrap_or("You");

        let text = msg.text.as_deref().unwrap_or("[media]");
        let time = msg.timestamp.as_ref()
            .map(|t| format_time_ago(t))
            .unwrap_or_default();

        println!(
            "{} {}",
            Theme::pink(sender),
            Theme::timestamp(&time)
        );
        println!("  {}", text);
        println!();
    }

    println!("{}", Theme::separator(60));
    println!(
        "{}",
        Theme::muted(&format!("Thread ID: {}", thread_id))
    );

    Ok(())
}

/// Print a thread summary for inbox view
fn print_thread_summary(index: usize, thread: &Thread) {
    // Get username for sending messages
    let username = thread.users.first().map(|u| u.username.as_str()).unwrap_or("unknown");

    // Use thread_title if available, otherwise use username
    let title = thread
        .thread_title
        .clone()
        .unwrap_or_else(|| username.to_string());

    let preview = thread
        .last_message_text
        .clone()
        .unwrap_or_else(|| "[media]".to_string());

    // Truncate preview
    let preview = if preview.chars().count() > 35 {
        format!("{}...", preview.chars().take(35).collect::<String>())
    } else {
        preview
    };

    // Unread indicator
    let unread = if thread.has_unread.unwrap_or(false) {
        format!("{}", Theme::unread_dot())
    } else {
        " ".to_string()
    };

    // Time
    let time = thread
        .last_message_timestamp
        .as_ref()
        .map(|t| format_time_ago(t))
        .unwrap_or_default();

    // Show: "1. Display Name (@username) 13d"
    println!(
        "{:>3}. {} {} {} {}",
        Theme::muted(&index.to_string()),
        Theme::orange(&title),
        Theme::username(&format!("@{}", username)),
        Theme::timestamp(&time),
        unread
    );
    println!("     {} {}", Theme::muted("└"), preview);
}

/// Format ISO timestamp to relative time
fn format_time_ago(timestamp: &str) -> String {
    // Parse "2026-01-14T12:33:38" format
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    // Simple parsing - extract date parts
    let parts: Vec<&str> = timestamp.split('T').collect();
    if parts.len() != 2 {
        return String::new();
    }

    let date_parts: Vec<u32> = parts[0].split('-').filter_map(|s| s.parse().ok()).collect();
    let time_parts: Vec<u32> = parts[1].split(':').filter_map(|s| s.parse().ok()).collect();

    if date_parts.len() != 3 || time_parts.len() < 2 {
        return String::new();
    }

    // Rough calculation (not accounting for timezones)
    let days_since_epoch = (date_parts[0] - 1970) * 365 + (date_parts[1] - 1) * 30 + date_parts[2];
    let secs = (days_since_epoch as u64) * 86400 + (time_parts[0] as u64) * 3600 + (time_parts[1] as u64) * 60;

    let msg_time = UNIX_EPOCH + Duration::from_secs(secs);
    let now = SystemTime::now();

    match now.duration_since(msg_time) {
        Ok(duration) => {
            let secs = duration.as_secs();
            if secs < 60 {
                "now".to_string()
            } else if secs < 3600 {
                format!("{}m", secs / 60)
            } else if secs < 86400 {
                format!("{}h", secs / 3600)
            } else {
                format!("{}d", secs / 86400)
            }
        }
        Err(_) => String::new(),
    }
}

/// Open chat by inbox number (1, 2, 3...)
pub async fn open_by_number(client: &ApiClient, number: usize) -> Result<()> {
    if number == 0 {
        println!("{} {}", Theme::cross(), Theme::error("Number must be 1 or greater"));
        return Ok(());
    }

    println!("{}", Theme::muted("Fetching inbox..."));

    let response = client.get_inbox(number as u32).await?;

    if !response.success {
        println!(
            "{} {}",
            Theme::cross(),
            Theme::error(&response.error.unwrap_or("Failed to fetch inbox".to_string()))
        );
        return Ok(());
    }

    let threads = response.threads.unwrap_or_default();

    if number > threads.len() {
        println!(
            "{} {}",
            Theme::cross(),
            Theme::error(&format!("No conversation at position {}. You have {} conversations.", number, threads.len()))
        );
        return Ok(());
    }

    // Get the thread at position (1-indexed)
    let thread = &threads[number - 1];
    let username = thread.users.first().map(|u| u.username.as_str()).unwrap_or("unknown");

    // Start chat with this user
    chat_with_user(client, username).await
}

/// Show thread by ID or @username
pub async fn show_thread_or_user(client: &ApiClient, target: &str, limit: u32) -> Result<()> {
    // Check if target starts with @ (username)
    if target.starts_with('@') {
        let username = &target[1..]; // Remove @ prefix
        show_thread_by_username(client, username, limit).await
    } else {
        // Assume it's a thread ID
        show_thread(client, target, limit).await
    }
}

/// Show thread by username (finds the thread first)
async fn show_thread_by_username(client: &ApiClient, username: &str, limit: u32) -> Result<()> {
    println!("{}", Theme::muted(&format!("Finding conversation with @{}...", username)));

    // Fetch inbox to find the thread
    let response = client.get_inbox(100).await?;

    if !response.success {
        println!(
            "{} {}",
            Theme::cross(),
            Theme::error(&response.error.unwrap_or("Failed to fetch inbox".to_string()))
        );
        return Ok(());
    }

    let threads = response.threads.unwrap_or_default();

    // Find thread with this username
    let thread = threads.iter().find(|t| {
        t.users.iter().any(|u| u.username.eq_ignore_ascii_case(username))
    });

    match thread {
        Some(t) => {
            show_thread(client, &t.id, limit).await
        }
        None => {
            println!(
                "{} {}",
                Theme::warn_icon(),
                Theme::warning(&format!("No conversation found with @{}", username))
            );
            Ok(())
        }
    }
}
