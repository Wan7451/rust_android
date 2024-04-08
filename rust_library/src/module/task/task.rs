use std::sync::Arc;
use futures_core::future::BoxFuture;
use crate::module::{Chain};
use reqwest::{RequestBuilder};
use crate::module::error::{Error, Result};
use crate::module::net::Response;

pub struct Task {
    chains: Vec<Arc<dyn Chain>>,
}


impl Task {
    pub fn new() -> Self {
        Task {
            chains: vec![],
        }
    }
    pub fn add_chain<T>(&mut self, chain: T) where T: Chain {
        self.chains.push(Arc::new(chain));
    }
    pub async fn run(&mut self, request_builder: RequestBuilder) -> BoxFuture<Result<Response>> {
        if let Some((curr, next)) = self.chains.split_first() {
            Box::pin(curr.process(request_builder, next))
        } else {
            return Box::pin(async {
                Err(Error::CustomError(String::from("aaaa")))
            });
        }
    }
}