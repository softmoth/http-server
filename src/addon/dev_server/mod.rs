use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;
use tokio::sync::broadcast::{self, Sender};

pub struct DevServer {
    pub kill_tx: Sender<()>,
}

impl DevServer {
    pub async fn new(root_dir: PathBuf) -> Self {
        let (kill_tx, mut kill_rx) = broadcast::channel(1024);
        let (tx, rx) = channel::<DebouncedEvent>();
        let watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2)).unwrap();

        tokio::select! {
            task = DevServer::watch(rx, root_dir, watcher) => task,
            kill_signal = kill_rx.recv() => panic!("Error!"),
        }

        Self { kill_tx }
    }

    async fn watch(
        rx: Receiver<DebouncedEvent>,
        root_dir: PathBuf,
        mut watcher: RecommendedWatcher,
    ) {
        tokio::spawn(async move {
            watcher
                .watch(root_dir, RecursiveMode::Recursive)
                .expect("Failed to initialize watcher.");

            loop {
                match rx.recv() {
                    Ok(event) => println!("{:?}", event),
                    Err(err) => panic!("{}", err.to_string()),
                }
            }
        })
        .await;
    }
}
