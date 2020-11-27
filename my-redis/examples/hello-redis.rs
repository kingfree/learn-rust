use mini_redis::{client, Result};

#[tokio::main]
pub async fn main() -> Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;
    let key = "hello";
    client.set(key, "world".into()).await?;
    let value = client.get(key).await?;
    println!("{} => {:?}", key, value);
    Ok(())
}
