use tokio::fs::File;
use tokio::io::{self,AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()>{

    let mut reader = "hello".as_bytes();

    let mut file = File::create("foo.txt").await?;

    io::copy(&mut reader, &mut file).await?;
    
    Ok(())
}