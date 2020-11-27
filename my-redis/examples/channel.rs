use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (mut tx, mut rx) = mpsc::channel(32);
    let mut tx2 = tx.clone();

    tokio::spawn(async move {
        tx.send("111").await;
    });
    
    tokio::spawn(async move {
        tx2.send("222").await;
    });
    
    while let Some(message) = rx.recv().await {
        println!("GOT = {}", message);
    }

}

