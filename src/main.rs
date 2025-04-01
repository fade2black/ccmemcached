use bytes::BytesMut;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}};
use ccmemcached::{errors::AppError, get_port};
use tracing::{info, instrument};

const HOST: &str = "127.0.0.1";
const DEFAULT_PORT: usize = 11211;

#[instrument]
#[tokio::main]
async fn main() -> Result<(), AppError> {
    tracing_subscriber::fmt::init();

    let port = get_port();
    let port = port.unwrap_or(DEFAULT_PORT);

    info!("listening on port {}", port);
    let listener = TcpListener::bind(format!("{}:{}", HOST, port)).await?;

    loop {
        info!("waiting for connection request");
        let (socket, _) = listener.accept().await?;
        info!("connection accepted");

        tokio::spawn (
            async move { process_connection(socket).await}
        );
    }  
}

#[instrument(skip(socket))]
async fn process_connection(mut socket: TcpStream) -> Result<(), AppError>{
    info!("processing connection");

    let mut buffer = BytesMut::with_capacity(512);

    loop {
        info!("reading");
        let size = socket.read_buf(&mut buffer).await?;

        if size == 0 {
            info!("read 0 bytes, so quiting");
            break;
        }

        info!("recieved payload '{:?}' of size {}", buffer, size);
    }

    info!("closing connection");
    socket.shutdown().await?;
    info!("connection closed");

    Ok(())
}

    // let slice = b"READ foobar";
    // let mut windows = slice.windows(2);
    // // for pairs in windows.clone() {
    // //     println!("{:?}", pairs);
    // // }

    // let res = windows.position(|window| window == b"\r\n");
    // println!("{:?}", res);
    
    // Ok(())  