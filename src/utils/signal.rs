use std::sync::Arc;
use tokio::task::JoinHandle;

pub async fn shutdown_signal(task_handle: Arc<JoinHandle<()>>) {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to hook Ctrl + C signal handler");
    println!("Received Ctrl + C Signal");
    task_handle.abort();
}
