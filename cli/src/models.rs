//! Data models matching the server API

#![allow(dead_code)]

use serde::{Deserialize, Serialize};

/// Login request body
#[derive(Debug, Serialize)]
pub struct LoginRequest {
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted_password: Option<String>,
}

/// Login response from server
#[derive(Debug, Deserialize)]
pub struct LoginResponse {
    pub success: bool,
    pub user: Option<User>,
    pub message: Option<String>,
}

/// Public key response for encryption
#[derive(Debug, Deserialize)]
pub struct PublicKeyResponse {
    pub public_key: String,
}

/// User info
#[derive(Debug, Deserialize)]
pub struct User {
    pub pk: String,
    pub username: String,
    pub full_name: Option<String>,
}

/// Health check response
#[derive(Debug, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub authenticated: bool,
    pub username: Option<String>,
}

/// Send message request
#[derive(Debug, Serialize)]
pub struct SendMessageRequest {
    pub text: String,
}

/// Send message response
#[derive(Debug, Deserialize)]
pub struct SendMessageResponse {
    pub success: bool,
    pub message: Option<Message>,
    pub error: Option<String>,
}

/// Inbox response
#[derive(Debug, Deserialize)]
pub struct InboxResponse {
    pub success: bool,
    pub threads: Option<Vec<Thread>>,
    pub error: Option<String>,
}

/// Thread response
#[derive(Debug, Deserialize)]
pub struct ThreadResponse {
    pub success: bool,
    pub thread: Option<Thread>,
    pub error: Option<String>,
}

/// A conversation thread
#[derive(Debug, Deserialize)]
pub struct Thread {
    pub id: String,
    pub users: Vec<User>,
    pub messages: Option<Vec<Message>>,
    pub thread_title: Option<String>,
    pub last_message_text: Option<String>,
    pub last_message_timestamp: Option<String>,
    pub has_unread: Option<bool>,
}

/// A direct message
#[derive(Debug, Deserialize)]
pub struct Message {
    pub id: String,
    pub text: Option<String>,
    pub timestamp: Option<String>,
    pub user_id: Option<String>,
    pub item_type: Option<String>,
}

/// Error response from server
#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    pub detail: String,
}

/// Search user response
#[derive(Debug, Deserialize)]
pub struct SearchUserResponse {
    pub success: bool,
    pub user: Option<UserDetails>,
}

/// Detailed user info from search
#[derive(Debug, Deserialize)]
pub struct UserDetails {
    pub pk: String,
    pub username: String,
    pub full_name: Option<String>,
    pub profile_pic_url: Option<String>,
    pub is_private: Option<bool>,
    pub is_verified: Option<bool>,
    pub follower_count: Option<u64>,
    pub following_count: Option<u64>,
}
