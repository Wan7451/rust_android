use std::fmt::Debug;

#[derive(Debug)]
pub struct Response {
    status: u16,
    body: String,
    url:String,
}


impl Response {
    pub(crate) async fn new(response: reqwest::Response) -> Self {
        let url = response.url().to_string();
        let mut status = response.status().as_u16();
        let body = if let Ok(body) = response.text().await {
            body
        } else {
            status = 500;
            "".to_string()
        };
        Response {
            status,
            body,
            url,
        }
    }

    pub fn status(&self) -> u16 {
        self.status
    }

    pub fn body(&self) -> String {
        self.body.clone()
    }

    pub fn url(&self) -> String {
        self.url.clone()
    }
}

