use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use tokio::task::JoinHandle;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::time::Duration;

use crate::addon::file_server::FileServer;

pub struct DevServer {
    pub file_server: Arc<FileServer>,
    watcher_task: Arc<JoinHandle<()>>,
}

impl DevServer {
    pub async fn new(root_dir: PathBuf) -> Self {
        let (tx, rx) = channel::<DebouncedEvent>();
        let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2)).unwrap();
        let file_server = Arc::new(FileServer::new(root_dir.clone()));
        let watcher_task: JoinHandle<()> = tokio::spawn(async move {
            watcher
                .watch(root_dir, RecursiveMode::Recursive)
                .expect("Failed to initialize watcher.");

            loop {
                match rx.recv() {
                    Ok(event) => println!("{:?}", event),
                    Err(err) => panic!("{}", err.to_string()),
                }
            }
        });

        Self { file_server, watcher_task: Arc::new(watcher_task) }
    }

    pub fn quit(&self) {
        self.watcher_task.abort();
    }
}
