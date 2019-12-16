use super::tam_tam::TamTam;

use reqwest::{Method, RequestBuilder, Response};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
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

impl TamTam {
    pub fn uploads(&self, r#type: UploadType) -> reqwest::Result<Response> {
        let path: String = format!("uploads");
        let req: RequestBuilder = self
            .request(Method::POST, &path)?
            .header(reqwest::header::CONTENT_TYPE, "multipart/form-data")
            .query(&[("type", r#type.value())]);
        Ok(req.send()?)
    }
}
