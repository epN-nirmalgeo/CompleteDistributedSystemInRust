use keyvaluedb::Result;
use tokio::net::TcpListener;

#[tokio::main]
pub async fn main() -> Result<()> {
    let listener = TcpListener::bind("localhost:6789").await?;
    loop {
        let (mut socket, _) = listener.accept().await?;
        println!("connection accepted {:?}", socket);
    }

    Ok(())
}