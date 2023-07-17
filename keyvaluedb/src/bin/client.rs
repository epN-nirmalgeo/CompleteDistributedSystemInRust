use connection::Connection;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> keyvaluedb::Result<()> {
    println!("Starting KeyValueDB!");

    let socket: TcpStream = TcpStream::connect("localhost:6379").await?;

    let mut connection = Connection::new(socket);


    //client::connect(addr)


    Ok(())
}
