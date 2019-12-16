use reqwest::{Method, RequestBuilder, Client};
use reqwest::header;

#[derive(Debug)]
pub struct TamTam {
    pub access_token: String,
    pub(crate) url: String,
    pub(crate) client: Client,
}

impl TamTam {
    pub fn new(access_token: String) -> TamTam {
        TamTam {
            url: String::from("https://botapi.tamtam.chat"),
            access_token,
            client: Client::new(),
        }
    }
}

// Private API
impl TamTam {
    // only for application/json and access-token type of requests
    pub(crate) fn request(&self, method: Method, path: &str) -> RequestBuilder {
        let full_link = format!("{}/{}", self.url, path);
        
        self.client
            .request(method, &full_link)
            .query(&[("access_token", &self.access_token)])
            .header(header::CONTENT_TYPE, "application/json")
    }
}
