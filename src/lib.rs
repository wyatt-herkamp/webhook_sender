use serde::{Deserialize, Serialize};
use thiserror::Error;
use async_trait::async_trait;

#[cfg(feature = "discord")]
pub mod discord;


#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type", content = "settings")]
pub enum WebhookConfigs {
    #[cfg(feature = "discord")]
    Discord(discord::DiscordWebhookConfig),
}

#[derive(Debug, Error)]
pub enum WebhookError {
    #[cfg(feature = "discord")]
    #[error(transparent)]
    #[cfg(feature = "discord")]
    Discord(#[from] discord::DiscordError),
    #[error("{0}")]
    Other(&'static str),
}

#[derive(Debug)]
#[non_exhaustive]
pub enum GenericWebhook {
    #[cfg(feature = "discord")]
    Discord(discord::DiscordWebhook),
}

#[async_trait]
impl Webhook for GenericWebhook {
    type Error = WebhookError;
    #[allow(unused_variables)]
    async fn send_dyn(&self, message: &WebhookMessage) -> Result<(), Self::Error> {
        match self {
            #[cfg(feature = "discord")]
            GenericWebhook::Discord(discord) => discord.send(message).await.map_err(WebhookError::from),
            _ => Err(WebhookError::Other("Not implemented")),
        }
    }
}

#[async_trait]
pub trait Webhook {
    type Error;

    async fn send_dyn(&self, message: &WebhookMessage) -> Result<(), Self::Error>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookMessage {
    pub title: String,
    pub content: String,
}

