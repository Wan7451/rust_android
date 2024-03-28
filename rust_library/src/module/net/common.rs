use std::fmt::{Display, Formatter};
use std::sync::Arc;
use dashmap::DashMap;
use once_cell::sync::OnceCell;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, IntoHeaderName};

use serde_json::Value;
use tokio::runtime::Runtime;

use crate::module::error::Error;

static CLIENTS: OnceCell<Arc<DashMap<String, Arc<HttpClient>>>> = OnceCell::new();
static RUNTIME: OnceCell<Runtime> = OnceCell::new();


pub fn init(base_url: &str, common_header: &str) -> Result<(), Error> {
    let clients = CLIENTS.get_or_init(|| {
        Arc::new(DashMap::new())
    });
    let client = HttpClient::new(base_url, common_header)?;
    clients.insert(base_url.to_string(), Arc::new(client));
    Ok(())
}

pub fn get_client(base_url: &str) -> Result<Arc<HttpClient>, Error> {
    let clients = CLIENTS.get_or_init(|| {
        Arc::new(DashMap::new())
    });
    let client = match clients.get(base_url) {
        Some(item) => item.clone(),
        None => {
            let client = HttpClient::new(base_url, "{}")?;
            let client = Arc::new(client);
            clients.insert(base_url.to_string(), client.clone());
            client.clone()
        }
    };
    Ok(client)
}

pub fn get_runtime() -> &'static Runtime {
    RUNTIME.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(4)
            .enable_all()
            .build()
            .unwrap()
    })
}


#[derive(Clone)]
pub struct HttpClient {
    pub client: reqwest::Client,
    pub headers: CommonHeader,
    pub base_url: String,
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

#[derive(Clone)]
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

impl Into<HeaderMap> for CommonHeader {
    fn into(self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        /*for (k, v) in self.params {
            headers.insert(HeaderName::from_static(&k), HeaderValue::from_static(&v));
        }*/
        headers
    }
}

impl Display for CommonHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self.params).unwrap_or(String::from("{}")))
    }
}