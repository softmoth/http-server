use notify::{watcher, DebouncedEvent, FsEventWatcher, Watcher};
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;

pub struct DevServer {
    pub root_dir: PathBuf,
    pub rx: Receiver<DebouncedEvent>,
    pub watcher: FsEventWatcher,
}

impl DevServer {
    pub fn new(root_dir: PathBuf) -> Self {
        let (tx, rx) = channel();
        let delay = Duration::from_millis(100);
        let mut watcher = watcher(tx, delay).unwrap();

        if let Err(err) = watcher.watch(&root_dir, notify::RecursiveMode::Recursive) {
            eprintln!("DevServerError: {}", err);
        }

        Self {
            root_dir,
            rx,
            watcher,
        }
    }

    pub fn watch(self) {
        loop {
            let recv = self.rx.recv();

            println!("{:?}", recv);
        }
    }
}
