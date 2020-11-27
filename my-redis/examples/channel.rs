use bytes::Bytes;
use mini_redis::{client, Result};
use tokio::sync::mpsc;

#[derive(Debug)]
enum Command {
    Get { key: String },
    Set { key: String, val: Bytes },
}

#[tokio::main]
async fn main() -> Result<()> {
    let (mut tx, mut rx) = mpsc::channel(32);

    let manager = tokio::spawn(async move {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();
        while let Some(cmd) = rx.recv().await {
            use Command::*;
            match cmd {
                Get { key } => {
                    client.get(&key).await;
                }
                Set { key, val } => {
                    client.set(&key, val).await;
                }
            }
        }
    });

    let mut tx2 = tx.clone();

    let t1 = tokio::spawn(async move {
        tx.send(Command::Get {
            key: "hello".to_string(),
        })
        .await
        .unwrap();
    });

    let t2 = tokio::spawn(async move {
        tx2.send(Command::Set {
            key: "foo".to_string(),
            val: "bar".into(),
        })
        .await
        .unwrap();
    });

    t1.await?;
    t2.await?;
    manager.await?;

    Ok(())
}
