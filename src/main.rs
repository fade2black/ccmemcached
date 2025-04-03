use std::{
    str,
    sync::{Arc, RwLock},
};

use bytes::BytesMut;
use ccmemcached::{
    Result, command::Command, get_port, hash_map_storage::HashMapStorage, protocol,
    storage::Storage,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};
use tracing::{info, instrument};

const HOST: &str = "127.0.0.1";
const DEFAULT_PORT: usize = 11211;

#[instrument]
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let storage = Arc::new(RwLock::new(HashMapStorage::new()));

    let port = get_port();
    let port = port.unwrap_or(DEFAULT_PORT);

    info!("listening on port {}", port);
    let listener = TcpListener::bind(format!("{}:{}", HOST, port)).await?;

    loop {
        info!("waiting for connection request");
        let (socket, _) = listener.accept().await?;
        info!("connection accepted");

        let s = Arc::clone(&storage);
        tokio::spawn(async move { process_connection(socket, s).await });
    }
}

#[instrument(skip(socket, storage))]
async fn process_connection<T>(mut socket: TcpStream, storage: Arc<RwLock<T>>) -> Result<()>
where
    T: Storage,
{
    info!("processing connection");

    let mut buffer = BytesMut::with_capacity(512);

    loop {
        info!("waiting for input");
        let size = socket.read_buf(&mut buffer).await?;

        // 0 bytes means connection is closed by client
        if size == 0 {
            info!("read 0 bytes, so quiting");
            break;
        }

        // Read command that ends with "\r\b"
        if let Some(pos) = buffer.windows(2).position(|window| window == b"\r\n") {
            let command_line = str::from_utf8(&buffer[..pos])?;
            let mut command = protocol::parse_command(command_line)?;

            // Read the data block if exists
            if let Command::Set(cmd) = &mut command {
                let mut data = vec![0u8; cmd.byte_count + 2]; // 2 bytes for "/r/n"
                info!("Waiting for data block");
                socket.read_exact(&mut data).await?;
                data.truncate(data.len() - 2);
                cmd.data = data;
            }

            let s = Arc::clone(&storage);
            let response = protocol::execute(command, s)?;
            socket.write_all(response.as_bytes()).await?;

            buffer.clear();
        }
    }

    socket.shutdown().await?;
    info!("connection closed");

    Ok(())
}
