use super::tam_tam::TamTam;

use serde::{Deserialize, Serialize};
use reqwest::Method;

#[derive(Debug, Deserialize, Serialize)]
pub struct TTBot {
    pub user_id: i64,
    pub name: String,
    pub username: Option<String>,
    pub avatar_url: Option<String>,
    pub full_avatar_url: Option<String>,
    pub commands: Option<Vec<TTCommand>>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TTCommand {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TTPhoto {
    pub url: Option<String>, // Any external image URL you want to attach
    pub token: Option<String>,
    pub photos: Option<PhotoToken>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PhotoToken {
    token: Option<String>,
}

impl TamTam {
    pub fn get_bot_info(&self) -> Result<TTBot, Box<dyn std::error::Error>> {
        let request: TTBot = self.request(Method::GET, "me").send()?.json()?;
        Ok(request)
    }

    pub fn edit_bot_info(&self, bot: TTBot) -> Result<(), reqwest::Error> {
        self.request(Method::PATCH, "me")
            .json(&bot)
            .query(&[("access_token", &self.access_token)])
            .send()?;

        Ok(())
    }
}
