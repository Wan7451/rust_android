use std::sync::Arc;

use futures_core::future::BoxFuture;
use log::Level;
use reqwest::RequestBuilder;

use crate::module::{Chain, get_client, LastNode, LoggingInterceptor, Request, RequestType};
use crate::module::error::{Error, Result};
use crate::module::net::Response;

pub struct Task<'a> {
    chains: Vec<Arc<dyn Chain>>,
    request: &'a Request,
}


impl<'a> Task<'a> {
    pub fn new(request: &'a Request) -> Self {
        Task {
            chains: vec![Arc::new(LoggingInterceptor::new(Level::Error)), Arc::new(LastNode)],
            request,
        }
    }

    pub fn add_chain<T>(&mut self, chain: T) where T: Chain {
        self.chains.push(Arc::new(chain));
    }

    fn generate_request(&self) -> Result<RequestBuilder> {
        let request = self.request;
        let client = get_client(&request.base_url())?;
        let headers = client.headers.clone().into();
        match request.request_type() {
            RequestType::Get => {
                Ok(client.client.get(request.url()).headers(headers))
            }
            RequestType::Post => {
                Ok(client.client.post(request.base_url()).headers(headers))
            }
            _ => {
                Err(Error::CustomError(String::from("not support")))
            }
        }
    }

    pub async fn run(&mut self) -> BoxFuture<Result<Response>> {
        if let Some((curr, next)) = self.chains.split_first() {
            if let Ok(request) = self.generate_request() {
                Box::pin(curr.process(request, next))
            } else {
                return Box::pin(async {
                    Err(Error::CustomError(String::from("create request error")))
                });
            }
        } else {
            return Box::pin(async {
                Err(Error::CustomError(String::from("task is empty")))
            });
        }
    }
}