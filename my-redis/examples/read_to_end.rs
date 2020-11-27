use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut f = File::open("Cargo.toml").await?;
    let mut buffer = Vec::new();

    f.read_to_end(&mut buffer).await?;

    println!("The bytes: {:?}", buffer);
    Ok(())
}