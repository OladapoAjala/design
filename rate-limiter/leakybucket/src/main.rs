use std::env;
use std::error::Error;
use tokio::io::{self};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio::time::{self, Duration};

const DEFAULT_RATE: u64 = 1;
const KEY: &str = "PROCESS_RATE";

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let sleep_interval = match env::var(KEY) {
        Ok(val) => {
            println!("sleep interval {}", val);
            val.parse().unwrap()
        }
        Err(e) => {
            println!("couldn't interpret {KEY}: {e}");
            DEFAULT_RATE
        }
    };

    let listener = TcpListener::bind("0.0.0.0:8081").await?;
    println!("Proxy listening on 0.0.0.0:8081");

    let (tx, mut rx) = mpsc::channel(100);
    tokio::spawn(async move {
        while let Some((client_stream, client_addr)) = rx.recv().await {
            if let Err(e) = handle_connection(client_stream, client_addr).await {
                println!("Error handling connection: {}", e);
            }
            time::sleep(Duration::from_secs(sleep_interval)).await;
        }
    });

    loop {
        let (client_stream, client_addr) = listener.accept().await?;
        println!("Client connected: {}", client_addr);
        if tx.send((client_stream, client_addr)).await.is_err() {
            println!("Channel send error");
            break;
        }
    }
    Ok(())
}

async fn handle_connection(
    mut client_stream: TcpStream,
    client_addr: std::net::SocketAddr,
) -> Result<(), Box<dyn Error>> {
    println!("Handling request from client: {}", client_addr);

    let mut server_stream = TcpStream::connect("127.0.0.1:3000").await?;
    println!("Connected to the gRPC server");

    match io::copy_bidirectional(&mut client_stream, &mut server_stream).await {
        Ok((client_to_server, server_to_client)) => {
            println!(
                "Forwarded {} bytes from client to server and {} bytes from server to client",
                client_to_server, server_to_client
            );
        }
        Err(_) => (),
    }
    Ok(())
}
