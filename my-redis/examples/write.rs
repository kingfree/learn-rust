use tokio::fs::File;
use tokio::io::{self, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut buffer = File::create("test.log").await?;
    buffer.write_all(b"shawanyiera").await?;
    Ok(())
}