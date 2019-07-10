use {
    reqwest::Response,
    reqwest::Client,
    reqwest::RequestBuilder,
    reqwest::Result,
};

use {
    std::collections::HashMap,
    std::clone::Clone,
};

#[derive(Debug)]
pub struct TamTam {
    url: String,
    access_token: String,
    client: Client
}

#[derive(Debug)]
pub struct EditInfoBuilder {
    request_builder: Result<RequestBuilder>
}

impl TamTam {
    pub fn new(access_token: String) -> TamTam {
        TamTam {
            url: String::from("https://botapi.tamtam.chat"),
            access_token,
            client: Client::new(),
        }
    }

    // TODO(uuttff8): Come up with new fn name
    pub fn get_info(&self) -> Result<Response> {
        let full_link = format!("{}/me", self.url);
        let resp = self.client.get(&full_link).query(&[("access_token", self.access_token.clone())]).send()?;
        Ok(resp)
    }

    pub fn change_info(&self) -> EditInfoBuilder {
        let full_link = format!("{}/me", self.url);
        let req_builder = self.client.patch(&full_link).query(&[("access_token", self.access_token.clone())]);
        EditInfoBuilder::new(Ok(req_builder))
    }
}


impl EditInfoBuilder {
    pub(crate) fn new(request_builder: Result<RequestBuilder>) -> EditInfoBuilder {
        EditInfoBuilder {
            request_builder,
        }
    }

    pub fn name(mut self, name: String) -> EditInfoBuilder {

        let mut map = HashMap::new();
        map.insert("name", name);

        if let Ok(ref mut req) = self.request_builder {
            req.json(&map);
        }

        self
    }
}
