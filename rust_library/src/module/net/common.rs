use std::fmt::{Display, Formatter};
use std::sync::Arc;
use dashmap::DashMap;
use once_cell::sync::OnceCell;

use serde_json::Value;

use crate::module::error::Error;


const BASE_URL: &str = "";
static CLIENTS: OnceCell<Arc<DashMap<String, Arc<HttpClient>>>> = OnceCell::new();

pub fn get_client(base_url: &str) -> Result<HttpClient, Error> {
    let clients = CLIENTS.get_or_init(|| {
        Arc::new(DashMap::new())
    });
    let client_arc = clients.get(base_url);
    if let Some(client) = client_arc {
        if let Some(client) = *client.value() {
            return Ok(client.clone());
        }
    }
    let client = HttpClient::new(base_url, "{}")?;
    clients.insert(base_url.to_string(), client.clone());
    Ok(client)
}


pub struct HttpClient {
    client: reqwest::Client,
    headers: CommonHeader,
    base_url: String,
}

impl HttpClient {
    pub fn new(base_url: &str, common_header: &str) -> Result<Self, Error> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10)).build()?;
        let headers = CommonHeader::new(common_header)?;

        Ok(HttpClient {
            client,
            headers,
            base_url: base_url.to_string(),
        })
    }
}

impl Display for HttpClient {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.base_url, self.headers)
    }
}


pub type KV = (String, String);

pub struct CommonHeader {
    params: Vec<KV>,
}

impl CommonHeader {
    pub fn new(headers: &str) -> Result<Self, serde_json::Error> {
        let json_value: Value = serde_json::from_str(headers)?;
        let mut params = vec![];
        if let Value::Object(map) = json_value {
            for (k, v) in map {
                params.push((k.to_string(), v.to_string()))
            }
        }
        Ok(CommonHeader { params })
    }
}

impl Display for CommonHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self.params).unwrap_or(String::from("{}")))
    }
}