use std::collections::HashMap;
use std::hash::Hash;
use std::path::PathBuf;
use serde_json::Value;

use tokio::runtime::Runtime;

use crate::module::error::Error;
use crate::module::{get_client, get_runtime};

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

struct Request {
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
    pub fn build(self) -> Request {
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
        let path = format!("{}{}", self.base_url, self.path);
        let client = get_client(&self.base_url)?;
        let headers = client.headers.clone().into();

        let result: String = match &self.request_type {
            RequestType::Get => {
                client.client.get(path).send().await?.text().await?
            }
            RequestType::Post => {
                client.client.post(path).headers(headers).send().await?.text().await?
            }
            _ => {
                String::from("")
            }
        };
        Ok(result)
    }
}

pub async fn get_request() -> Result<String, reqwest::Error> {
    let mut map = HashMap::new();
    map.insert("lang", "rust");
    map.insert("body", "json");

    let client = reqwest::Client::new();
    let res = client.post("http://httpbin.org/post")
        //.json(&map)
        .send()
        .await?.text().await;
    println!("{res:#?}");
    res
}


pub fn test_request<F>(cd: F) where F: FnOnce(String) {
   let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .unwrap();
    runtime.block_on(async {
        let a = aaaaaaa().await;
        match a {
            Ok(a) => {
                //cd(a.clone());
                //println!("{a}");
            }
            Err(e) => {
                //cd(e.to_string());
                println!("{e}");
            }
        }
    })
}

async fn aaaaaaa() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{resp:#?}");
    Ok(())
}


