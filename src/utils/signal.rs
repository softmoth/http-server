use tokio::sync::broadcast::Sender;

pub async fn shutdown_signal(kill_tx: Sender<()>) {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to hook Ctrl + C signal handler");
    println!("Received Ctrl + C Signal");
    kill_tx.send(());
}
