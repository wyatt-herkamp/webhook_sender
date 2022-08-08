use crate::Serialize;

#[derive(Serialize, Debug, Default)]
pub struct DiscordMessage<'content> {
    pub content: Option<&'content str>,
    pub username: Option<&'content str>,
    pub avatar_url: Option<&'content str>,
    pub tts: bool,
    pub embeds: Vec<Embed<'content>>,
}

#[derive(Serialize, Debug)]
pub struct Embed<'content> {
    pub title: Option<&'content str>,
    #[serde(rename = "type")]
    pub embed_type: &'content str,
    pub description: Option<&'content str>,
    pub url: Option<&'content str>,
    pub timestamp: Option<&'content str>,
    pub color: Option<&'content str>,
}

impl Default for Embed<'_> {
    fn default() -> Self {
        Embed {
            title: None,
            embed_type: "rich",
            description: None,
            url: None,
            timestamp: None,
            color: None,
        }
    }
}
