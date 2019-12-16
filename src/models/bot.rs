use serde::{Deserialize, Serialize};

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
