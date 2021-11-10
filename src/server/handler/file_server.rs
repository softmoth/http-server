use http::response::Builder as HttpResponseBuilder;
use http::StatusCode;
use hyper::body::{aggregate, Buf};
use hyper::{Body, Method, Request};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::addon::file_server::FileServer;

use super::Handler;

pub struct FileServerHandler {
    file_server: Arc<FileServer>,
}

impl FileServerHandler {
    pub fn new(file_server: FileServer) -> Self {
        let file_server = Arc::new(file_server);

        FileServerHandler { file_server }
    }

    pub fn handle(&self) -> Handler {
        let file_server = Arc::clone(&self.file_server);

        Box::new(move |request: Arc<Mutex<Request<Body>>>| {
            let file_server = Arc::clone(&file_server);
            let request = Arc::clone(&request);

            Box::pin(async move {
                let file_server = Arc::clone(&file_server);
                let request = Arc::clone(&request);
                let mut request_lock = request.lock().await;
                let req_path = request_lock.uri().to_string();
                let req_method = request_lock.method();

                match *req_method {
                    Method::GET => {
                        return file_server
                            .resolve(req_path)
                            .await
                            .map_err(|e| {
                                HttpResponseBuilder::new()
                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                    .body(Body::from(e.to_string()))
                                    .expect("Unable to build response")
                            })
                            .unwrap();
                    }
                    Method::POST => {
                        let mut buffer: Vec<u8> = Vec::new();
                        let body = request_lock.body_mut();
                        let mut buffer_cursor = aggregate(body).await.unwrap();

                        while buffer_cursor.has_remaining() {
                            buffer.push(buffer_cursor.get_u8());
                        }

                        return file_server
                            .create(req_path, buffer)
                            .await
                            .map_err(|e| {
                                HttpResponseBuilder::new()
                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                    .body(Body::from(e.to_string()))
                                    .expect("Unable to build response")
                            })
                            .unwrap();
                    }
                    _ => HttpResponseBuilder::new()
                        .status(StatusCode::METHOD_NOT_ALLOWED)
                        .body(Body::empty())
                        .expect("Unable to build response"),
                }
            })
        })
    }
}
