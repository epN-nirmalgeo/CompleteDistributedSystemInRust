use bytes::BytesMut;
use tokio::io::BufWriter;
use tokio::net::TcpStream;
use keyvaluedb::Result;

pub struct Client {
    connection: Connection,
}


#[derive(Debug)]
pub struct Connection {
    stream: BufWriter<TcpStream>,
    buffer: BytesMut,
}

impl Connection {
    pub fn new(socket: TcpStream) -> Self {
        Self {
            stream: BufWriter::new(socket),
            // 100 KB
            buffer: BytesMut::with_capacity(100 * 1024),
        }
    }

    pub fn read_frame(&mut self) -> Result<()> {
        self.stream.read_buf(&mut self.buffer).await?;
        Ok(())
    }
}