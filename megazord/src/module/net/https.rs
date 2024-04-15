use std::collections::HashMap;

use serde_json::{json, Value};

use crate::module::{get_runtime, Task};
use crate::module::error::Error;

#[derive(Copy, Clone)]
pub enum RequestType {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
    Options,
    Trace,
}

pub struct Request {
    base_url: String,
    params: HashMap<String, String>,
    headers: HashMap<String, String>,
    path: String,
    request_type: RequestType,
}

pub struct RequestBuilder {
    base_url: String,
    params: HashMap<String, String>,
    headers: HashMap<String, String>,
    path: String,
    request_type: RequestType,
}

impl RequestBuilder {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            params: HashMap::new(),
            headers: HashMap::new(),
            path: "".to_string(),
            request_type: RequestType::Post,
        }
    }
    pub fn path(mut self, path: &str) -> Self {
        self.path = path.to_string();
        self
    }
    pub fn params(mut self, params: &str) -> Self {
        let params = serde_json::from_str(params);
        if let Ok(params) = params {
            let params: Value = params;
            if let Value::Object(params) = params {
                for (key, value) in params {
                    self.params.insert(key.to_string(), value.to_string());
                }
            }
        }
        self
    }
    pub fn headers<K, V>(mut self, key: K, value: V) -> Self where K: ToString, V: ToString {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }
    pub fn get<F>(mut self, callback: F) -> Result<String, Error> where F: FnOnce(Result<String, Error>) {
        self.request_type = RequestType::Get;
        let request = self.build();
        request.run(callback);
        Ok(String::from(""))
    }
    pub fn post<F>(mut self, callback: F) -> Result<String, Error> where F: FnOnce(Result<String, Error>) {
        self.request_type = RequestType::Post;
        let request = self.build();
        request.run(callback);
        Ok(String::from(""))
    }
    fn build(self) -> Request {
        Request {
            base_url: self.base_url,
            params: self.params,
            headers: self.headers,
            path: self.path,
            request_type: self.request_type,
        }
    }
}

impl Request {
    pub fn run<F>(self, callback: F) where F: FnOnce(Result<String, Error>) {
        get_runtime().block_on(async {
            let result = self.action().await;
            callback(result);
        });
    }
    async fn action(&self) -> Result<String, Error> {
        let response = Task::new(self).run().await.await?;
        let result = response.body();
        Ok(result)
    }

    pub fn url(&self) -> String {
        format!("{}{}", self.base_url, self.path)
    }

    pub fn base_url(&self) -> String {
        self.base_url.clone()
    }

    pub fn params(&self) -> Value {
        let mut params = json!({});
        for (key, value) in &self.params {
            params[key] = json!(value);
        }
        params
    }

    pub fn request_type(&self) -> RequestType {
        self.request_type.clone()
    }
}
