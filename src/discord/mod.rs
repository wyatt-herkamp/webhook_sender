pub mod content;

use reqwest::{Body, Method, Request, RequestBuilder};
use thiserror::Error;
use crate::{Deserialize, Serialize, Webhook, WebhookMessage};
use crate::discord::content::{DiscordMessage, Embed};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DiscordWebhookConfig {
    /// The URL of the webhook.
    pub webhook_url: String,
    //TODO contain settings for setting default values for the webhook\
    pub username: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Debug)]
pub struct DiscordWebhook {
    pub config: DiscordWebhookConfig,
}

#[cfg(feature = "async-trait")]
#[async_trait::async_trait]
impl Webhook for DiscordWebhook {
    type Error = DiscordError;

    async fn send_dyn(&self, message: &WebhookMessage) -> Result<(), Self::Error> {
        Self::send(&self, message).await
    }
}

impl DiscordWebhook {
    /// Send a generic message to a discord webhook.
    pub async fn send(&self, message: &WebhookMessage) -> Result<(), DiscordError> {
        let embed = Embed {
            title: Some(&message.title),
            description: Some(&message.content),
            ..Embed::default()
        };
        let message = DiscordMessage {
            embeds: vec![embed],
            avatar_url: self.config.avatar_url.as_ref().and_then(|url| Some(url.as_str())),
            username: self.config.username.as_ref().and_then(|username| Some(username.as_str())),
            ..DiscordMessage::default()
        };
        self.send_discord(message).await
    }
    /// Send a Discord message to a discord webhook.
    pub async fn send_discord(&self, message: DiscordMessage<'_>) -> Result<(), DiscordError> {
        let body = serde_json::to_string(&message)?;

        let client = reqwest::Client::new();
        let response = client.request(Method::POST, &self.config.webhook_url).header("content-type", "application/json").body(Body::from(body)).build()?;
        let response = client.execute(response).await?;
        if !response.status().is_success() {
            return Err(DiscordError::HttpError(response.status()));
        }
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum DiscordError {
    #[error("HTTP error: {0}")]
    HttpError(reqwest::StatusCode),
    #[error("{0}")]
    SerializeError(#[from] serde_json::Error),
    #[error("{0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("{0}")]
    Other(&'static str),
}

impl TryFrom<DiscordWebhookConfig> for DiscordWebhook {
    type Error = DiscordError;

    fn try_from(value: DiscordWebhookConfig) -> Result<Self, Self::Error> {
        Ok(DiscordWebhook {
            config: value
        })
    }
}



