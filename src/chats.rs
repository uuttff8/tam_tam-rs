use reqwest::{RequestBuilder, Response, Method};
use super::tam_tam::TamTam;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
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

impl TamTam {
    pub fn get_all_chats(&self) -> reqwest::Result<Response> {
        Ok(self.request(Method::GET, "chats").send()?)
    }

    pub fn get_chat_by_id(&self, id: usize) -> reqwest::Result<Response> {
        let path = format!("chats/{}", id.to_string());
        Ok(self.request(Method::GET, &path).send()?)
    }

    // TODO(uuttff8): support for title
    pub fn edit_chat_info(&self, id: usize, title: &str) -> reqwest::Result<Response> {
        let path = format!("chats/{}", id.to_string());
        let req = self
            .request(Method::PATCH, &path)
            .query(&["title", title])
            .send()?;
        Ok(req)
    }

    pub fn send_action(&self, id: usize, action: SenderAction) -> reqwest::Result<Response> {
        let path = format!("chats/{}", id.to_string());
        let req: RequestBuilder = self
            .request(Method::PATCH, &path)
            .query(&[("action", action.value())]);
        Ok(req.send()?)
    }

    pub fn get_members(&self, id: usize) -> reqwest::Result<Response> {
        let path = format!("chats/{}/members", id.to_string());
        let req: RequestBuilder = self.request(Method::PATCH, &path);
        Ok(req.send()?)
    }

    pub fn leave_chat(&self, id: usize) -> reqwest::Result<Response> {
        let path: String = format!("chats/{}/members/me", id.to_string());
        let req: RequestBuilder = self.request(Method::DELETE, &path);
        Ok(req.send()?)
    }

    pub fn add_members(&self, id: usize, user_ids: Vec<i64>) -> reqwest::Result<Response> {
        let path: String = format!("chats/{}/members", id.to_string());
        let req: RequestBuilder = self
            .request(Method::POST, &path)
            .query(&[("user_ids", user_ids)]);
        Ok(req.send()?)
    }

    pub fn remove_member(&self, id: usize, user_id: i64) -> reqwest::Result<Response> {
        let path: String = format!("chats/{}/members", id.to_string());
        let req: RequestBuilder = self
            .request(Method::DELETE, &path)
            .query(&[("user_ids", user_id)]);
        Ok(req.send()?)
    }
}