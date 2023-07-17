use mini_redis::client;
use tokio::net::TcpStream;
use std::io::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting KeyValueDB!");

    let socket: TcpStream = TcpStream::connect("localhost:6379").await?;

    client::connect(addr)


    Ok(())
}
