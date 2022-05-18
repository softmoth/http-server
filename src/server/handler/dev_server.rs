use async_trait::async_trait;
use http::response::Builder as HttpResponseBuilder;
use http::StatusCode;
use hyper::{Body, Method, Request};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::addon::dev_server::DevServer;

use super::RequestHandler;

pub struct DevServerHandler {
    dev_server: Arc<DevServer>,
}

impl DevServerHandler {
    pub fn new(dev_server: DevServer) -> Self {
        println!("builds dev server handler");
        let dev_server = Arc::new(dev_server);

        Self { dev_server }
    }

    pub fn quit(&self) {
        self.dev_server.quit();
    }
}

#[async_trait]
impl RequestHandler for DevServerHandler {
    async fn handle(&self, req: Arc<Mutex<Request<Body>>>) -> Arc<Mutex<http::Response<Body>>> {
        println!("GOT IT");
        let request_lock = req.lock().await;
        let req_path = request_lock.uri().to_string();
        let req_method = request_lock.method();

        if req_method == Method::GET {
            let response = self.dev_server.file_server.resolve(req_path).await.unwrap();

            return Arc::new(Mutex::new(response));
        }

        Arc::new(Mutex::new(
            HttpResponseBuilder::new()
                .status(StatusCode::METHOD_NOT_ALLOWED)
                .body(Body::empty())
                .expect("Unable to build response"),
        ))
    }
}
