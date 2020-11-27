use bytes::Bytes;
use mini_redis::{client, Result};
use tokio::sync::{mpsc, oneshot};

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Bytes,
        resp: Responder<()>,
    },
}

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[tokio::main]
async fn main() -> Result<()> {
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    let manager = tokio::spawn(async move {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();
        while let Some(cmd) = rx.recv().await {
            use Command::*;
            match cmd {
                Get { key, resp } => {
                    resp.send(client.get(&key).await).unwrap();
                }
                Set { key, val, resp } => {
                    resp.send(client.set(&key, val).await).unwrap();
                }
            }
        }
    });

    let t1 = tokio::spawn(async move {
        let (r_tx, r_rx) = oneshot::channel();
        tx.send(Command::Get {
            key: "hello".to_string(),
            resp: r_tx,
        })
        .await
        .unwrap();
        println!("GOT = {:?}", r_rx.await);
    });

    let t2 = tokio::spawn(async move {
        let (r_tx, r_rx) = oneshot::channel();
        tx2.send(Command::Set {
            key: "foo".to_string(),
            val: "bar".into(),
            resp: r_tx,
        })
        .await
        .unwrap();
        println!("GOT = {:?}", r_rx.await);
    });

    t1.await?;
    t2.await?;
    manager.await?;

    Ok(())
}
