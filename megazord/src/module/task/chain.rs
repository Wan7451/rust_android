use std::sync::Arc;

use async_trait::async_trait;
use futures_core::future::BoxFuture;
use futures_util::future::ok;
use log::Level;
use reqwest::RequestBuilder;

use crate::module::error::Error;
use crate::module::error::Error::CustomError;
use crate::module::error::Result;
use crate::module::net::Response;

#[async_trait]
pub trait Chain: 'static + Send + Sync {
    async fn process(&self, request: RequestBuilder, chains: &[Arc<dyn Chain>]) -> Result<Response>;
}

// #[async_trait]
// impl<F> Chain for F
//     where F: 'static
//     + Send
//     + Sync
//     + for<'a> Fn(RequestBuilder, Next<'a>) -> BoxFuture<'a, Result<Response>> {
//     async fn process(&self, request: RequestBuilder, next: Next<'_>) -> Result<Response> {
//         self(request, next).await
//     }
// }

pub struct LoggingInterceptor {
    level: Level,
}

impl LoggingInterceptor {
    pub fn new(level: Level) -> Self {
        Self {
            level,
        }
    }
}

#[async_trait]
impl Chain for LoggingInterceptor {
    async fn process(&self, request: RequestBuilder, chains: &[Arc<dyn Chain>]) -> Result<Response> {
        log::log!(self.level, "{:?}", request);
        if let Some((curr, next)) = chains.split_first() {
            let response = curr.process(request, next).await?;
            log::log!(self.level, "{:?}", response);
            Ok(response)
        } else {
            Err(CustomError(String::from("tasks异常")))
        }
    }
}

pub struct JsonInterceptor;

// #[async_trait]
// impl Chain for JsonInterceptor {
//     async fn process(&self, request: RequestBuilder, chains: &[Arc<dyn Chain>]) -> Result<Response> {
//     }
// }


pub struct LastNode;

#[async_trait]
impl Chain for LastNode {
    async fn process(&self, request: RequestBuilder, chains: &[Arc<dyn Chain>]) -> Result<Response> {
        Ok(Response::new(request.send().await?).await)
    }
}