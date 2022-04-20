use async_trait::async_trait;
use hyper::{Body, Request};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

use crate::addon::dev_server::DevServer;

use super::RequestHandler;

pub struct DevServerHandler;

impl DevServerHandler {
    pub fn new(mut dev_server: DevServer) -> Self {
        Self
    }
}

#[async_trait]
impl RequestHandler for DevServerHandler {
    async fn handle(&self, req: Arc<Mutex<Request<Body>>>) -> Arc<Mutex<http::Response<Body>>> {
        let response = http::Response::default();

        Arc::new(Mutex::new(response))
    }
}
