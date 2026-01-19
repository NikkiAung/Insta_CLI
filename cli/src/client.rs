//! HTTP client for communicating with the Instagram DM server

use anyhow::{Context, Result};
use reqwest::Client;

use crate::crypto::encrypt_password;
use crate::models::*;

/// Default server URL
const DEFAULT_SERVER_URL: &str = "http://localhost:8000";

/// Instagram DM API client
pub struct ApiClient {
    client: Client,
    base_url: String,
}

impl ApiClient {
    /// Create a new API client
    pub fn new(base_url: Option<&str>) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.unwrap_or(DEFAULT_SERVER_URL).to_string(),
        }
    }

    /// Check server health and authentication status
    pub async fn health(&self) -> Result<HealthResponse> {
        let url = format!("{}/health", self.base_url);
        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to connect to server")?;

        resp.json()
            .await
            .context("Failed to parse health response")
    }

    /// Get the server's public key for password encryption
    pub async fn get_public_key(&self) -> Result<String> {
        let url = format!("{}/auth/public-key", self.base_url);
        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to fetch public key")?;

        let key_resp: PublicKeyResponse = resp
            .json()
            .await
            .context("Failed to parse public key response")?;

        Ok(key_resp.public_key)
    }

    /// Login with encrypted password
    pub async fn login(&self, username: &str, password: &str) -> Result<LoginResponse> {
        // First, get the server's public key
        let public_key = self.get_public_key().await?;

        // Encrypt the password
        let encrypted_password = encrypt_password(password, &public_key)?;

        // Send login request with encrypted password
        let url = format!("{}/auth/login", self.base_url);
        let req = LoginRequest {
            username: username.to_string(),
            password: None,
            encrypted_password: Some(encrypted_password),
        };

        let resp = self
            .client
            .post(&url)
            .json(&req)
            .send()
            .await
            .context("Failed to send login request")?;

        if resp.status().is_success() {
            resp.json()
                .await
                .context("Failed to parse login response")
        } else {
            let error: ErrorResponse = resp
                .json()
                .await
                .unwrap_or(ErrorResponse {
                    detail: "Unknown error".to_string(),
                });
            anyhow::bail!("Login failed: {}", error.detail)
        }
    }

    /// Logout from Instagram
    pub async fn logout(&self) -> Result<()> {
        let url = format!("{}/auth/logout", self.base_url);
        self.client
            .post(&url)
            .send()
            .await
            .context("Failed to logout")?;
        Ok(())
    }

    /// Get inbox (list of conversation threads)
    pub async fn get_inbox(&self, limit: u32) -> Result<InboxResponse> {
        let url = format!("{}/inbox?limit={}", self.base_url, limit);
        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to fetch inbox")?;

        if resp.status().is_success() {
            resp.json().await.context("Failed to parse inbox response")
        } else if resp.status().as_u16() == 401 {
            anyhow::bail!("Not authenticated. Please login first.")
        } else {
            anyhow::bail!("Failed to fetch inbox: {}", resp.status())
        }
    }

    /// Get a specific thread with messages
    pub async fn get_thread(&self, thread_id: &str, limit: u32) -> Result<ThreadResponse> {
        let url = format!("{}/thread/{}?limit={}", self.base_url, thread_id, limit);
        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to fetch thread")?;

        if resp.status().is_success() {
            resp.json().await.context("Failed to parse thread response")
        } else if resp.status().as_u16() == 401 {
            anyhow::bail!("Not authenticated. Please login first.")
        } else {
            anyhow::bail!("Failed to fetch thread: {}", resp.status())
        }
    }

    /// Send a message to an existing thread
    pub async fn send_to_thread(&self, thread_id: &str, text: &str) -> Result<SendMessageResponse> {
        let url = format!("{}/thread/{}/send", self.base_url, thread_id);
        let req = SendMessageRequest {
            text: text.to_string(),
        };

        let resp = self
            .client
            .post(&url)
            .json(&req)
            .send()
            .await
            .context("Failed to send message")?;

        if resp.status().is_success() {
            resp.json()
                .await
                .context("Failed to parse send response")
        } else if resp.status().as_u16() == 401 {
            anyhow::bail!("Not authenticated. Please login first.")
        } else {
            anyhow::bail!("Failed to send message: {}", resp.status())
        }
    }

    /// Send a message to a user by username
    pub async fn send_to_user(&self, username: &str, text: &str) -> Result<SendMessageResponse> {
        let url = format!("{}/send/{}", self.base_url, username);
        let req = SendMessageRequest {
            text: text.to_string(),
        };

        let resp = self
            .client
            .post(&url)
            .json(&req)
            .send()
            .await
            .context("Failed to send message")?;

        if resp.status().is_success() {
            resp.json()
                .await
                .context("Failed to parse send response")
        } else if resp.status().as_u16() == 401 {
            anyhow::bail!("Not authenticated. Please login first.")
        } else {
            anyhow::bail!("Failed to send message: {}", resp.status())
        }
    }

    /// Search for a user by username
    pub async fn search_user(&self, username: &str) -> Result<SearchUserResponse> {
        let url = format!("{}/user/{}", self.base_url, username);
        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to search user")?;

        if resp.status().is_success() {
            resp.json()
                .await
                .context("Failed to parse search response")
        } else if resp.status().as_u16() == 401 {
            anyhow::bail!("Not authenticated. Please login first.")
        } else if resp.status().as_u16() == 404 {
            anyhow::bail!("User '{}' not found", username)
        } else {
            anyhow::bail!("Failed to search user: {}", resp.status())
        }
    }
}
