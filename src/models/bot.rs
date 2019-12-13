use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TTBot {
    user_id: i64,
    name: String,
    username: Option<String>,
    avatar_url: Option<String>,
    full_avatar_url: Option<String>,
    commands: Option<Vec<TTCommand>>,
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TTCommand {
    name: String,
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TTPhoto {
    url: Option<String>, // Any external image URL you want to attach
    token: Option<String>,
    photos: Option<PhotoToken>,
}

#[derive(Debug, Deserialize)]
pub struct PhotoToken {
    token: Option<String>,
}
