#![allow(dead_code)]
use {
    reqwest::{Client, Method, RequestBuilder, Response},
    serde::Deserialize,
    std::io,
};

use crate::models::bot::TTBot;

#[derive(Deserialize, Debug)]
pub struct ChatJson {
    pub chats: Vec<ChatJsonArray>,
}

#[derive(Deserialize, Debug)]
pub struct ChatJsonArray {
    pub chat_id: usize,
    pub r#type: String,
    pub status: String,
    pub last_event_time: usize,
    pub participants_count: usize,
    pub is_public: bool,
}

#[derive(Debug)]
pub struct TamTam {
    url: String,
    access_token: String,
    client: Client,
}

impl TamTam {
    pub fn new(access_token: String) -> TamTam {
        TamTam {
            url: String::from("https://botapi.tamtam.chat"),
            access_token,
            client: Client::new(),
        }
    }

    pub fn get_info(&self) -> Result<TTBot, Box<dyn std::error::Error>> {
        let request: TTBot = self.request(Method::GET, "me")?.send()?.json()?;
        Ok(request)
    }

    pub fn change_bot_info(&self) -> Result<(), io::Error> {
        let full_link = format!("{}/me", self.url);
        self.client
            .request(Method::PATCH, &full_link)
            .query(&[("access_token", self.access_token.clone())]);

        Ok(())
    }

    pub fn edit_access_token(mut self, access_token: String) -> TamTam {
        self.access_token = access_token;
        self
    }

    // Chats
    pub fn get_all_chats(&self) -> reqwest::Result<Response> {
        Ok(self.request(Method::GET, "chats")?.send()?)
    }

    pub fn get_chat_by_id(&self, id: usize) -> reqwest::Result<Response> {
        let path = format!("chats/{}", id.to_string());
        Ok(self.request(Method::GET, &path)?.send()?)
    }

    // TODO(uuttff8): support for title
    pub fn edit_chat_info(&self, id: usize, title: &str) -> reqwest::Result<Response> {
        let path = format!("chats/{}", id.to_string());
        let req = self
            .request(Method::PATCH, &path)?
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .query(&["title", title])
            .send()?;
        Ok(req)
    }

    pub fn send_action(&self, id: usize, action: SenderAction) -> reqwest::Result<Response> {
        let path = format!("chats/{}", id.to_string());
        let req: RequestBuilder = self
            .request(Method::PATCH, &path)?
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .query(&[("action", action.value())]);
        Ok(req.send()?)
    }

    pub fn get_members(&self, id: usize) -> reqwest::Result<Response> {
        let path = format!("chats/{}/members", id.to_string());
        let req: RequestBuilder = self.request(Method::PATCH, &path)?;
        Ok(req.send()?)
    }

    pub fn leave_chat(&self, id: usize) -> reqwest::Result<Response> {
        let path: String = format!("chats/{}/members/me", id.to_string());
        let req: RequestBuilder = self.request(Method::DELETE, &path)?;
        Ok(req.send()?)
    }

    pub fn add_members(&self, id: usize, user_ids: Vec<i64>) -> reqwest::Result<Response> {
        let path: String = format!("chats/{}/members", id.to_string());
        let req: RequestBuilder = self
            .request(Method::POST, &path)?
            .query(&[("user_ids", user_ids)]);
        Ok(req.send()?)
    }

    pub fn remove_member(&self, id: usize, user_id: i64) -> reqwest::Result<Response> {
        let path: String = format!("chats/{}/members", id.to_string());
        let req: RequestBuilder = self
            .request(Method::DELETE, &path)?
            .query(&[("user_ids", user_id)]);
        Ok(req.send()?)
    }

    // MARK: - Messages

    //TODO(uuttff8): implement
    fn get_messages(&self) {
        unimplemented!();
    }

    fn send_message(&self) {
        unimplemented!();
    }
    fn edit_message(&self) {
        unimplemented!();
    }
    fn delete_message(&self) {
        unimplemented!();
    }
    fn answer_on_callback(&self) {
        unimplemented!();
    }
    // MARK: - Subscriptions
    //TODO(uuttff8): implement
    fn get_subscription(&self) {
        unimplemented!();
    }

    fn subscribe(&self) {
        unimplemented!();
    }

    fn unsubscribe(&self) {
        unimplemented!();
    }

    fn get_updates(&self) {
        unimplemented!();
    }

    // MARK: - Upload

    pub fn uploads(&self, r#type: UploadType) -> reqwest::Result<Response> {
        let path: String = format!("uploads");
        let req: RequestBuilder = self
            .request(Method::POST, &path)?
            .header(reqwest::header::CONTENT_TYPE, "multipart/form-data")
            .query(&[("type", r#type.value())]);
        Ok(req.send()?)
    }

    fn request(&self, method: Method, path: &str) -> reqwest::Result<RequestBuilder> {
        let full_link = format!("{}/{}", self.url, path);
        let resp = self
            .client
            .request(method, &full_link)
            .query(&[("access_token", self.access_token.clone())]);
        Ok(resp)
    }
}
#[derive(Deserialize)]
pub enum UploadType {
    Photo,
    Video,
    Audio,
    File,
}

impl UploadType {
    pub fn value(&self) -> String {
        match *self {
            UploadType::Photo => String::from("photo"),
            UploadType::Video => String::from("video"),
            UploadType::Audio => String::from("audio"),
            UploadType::File => String::from("file"),
        }
    }
}

#[derive(Deserialize)]
pub enum SenderAction {
    TypingOn,
    TypingOff,
    SendingPhoto,
    SendingVideo,
    SendingAudio,
    MarkSeen,
}

impl SenderAction {
    fn value(&self) -> String {
        match *self {
            SenderAction::MarkSeen => String::from("mark_seen"),
            SenderAction::SendingAudio => String::from("sending_audio"),
            SenderAction::SendingPhoto => String::from("sending_photo"),
            SenderAction::SendingVideo => String::from("sending_video"),
            SenderAction::TypingOff => String::from("typing_off"),
            SenderAction::TypingOn => String::from("typing_on"),
        }
    }
}
