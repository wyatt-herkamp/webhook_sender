use std::fs::OpenOptions;
use webhook_sender::discord::{DiscordWebhook, DiscordWebhookConfig};
use webhook_sender::WebhookMessage;

#[tokio::test]
pub async fn test() {
    let value = env!("DISCORD_URL");
    let config = DiscordWebhookConfig {
        webhook_url: value.to_string(),
        username: Some("Ferris".to_string()),
        avatar_url: Some("https://rustacean.net/assets/rustacean-flat-happy.png".to_string()),
    };
    let webhook = DiscordWebhook::try_from(config).unwrap();
    webhook.send(&WebhookMessage {
        title: "Rust".to_string(),
        content: "Rusty Content".to_string(),
    }).await.unwrap();
}

#[tokio::test]
pub async fn as_config() {
    let config: DiscordWebhookConfig = serde_json::from_reader(OpenOptions::new().read(true).open("tests/discord.json").unwrap()).unwrap();
    let webhook = DiscordWebhook::try_from(config).unwrap();
    webhook.send(&WebhookMessage {
        title: "Rust".to_string(),
        content: "Rusty Content".to_string(),
    }).await.unwrap();
}