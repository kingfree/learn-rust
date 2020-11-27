use tokio::fs::File;
use tokio::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut reader: &[u8] = b"hello";
    let mut file = File::create("copy.log").await?;
    io::copy(&mut reader, &mut file).await?;
    Ok(())
}