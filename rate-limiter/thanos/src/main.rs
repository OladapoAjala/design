use memcache::Client;
use std::error::Error;
use tokio::io::{self, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    println!("Proxy listening on 0.0.0.0:3000");

    let memcached = Client::connect("memcache://cache-mcrouter:5000")?;

    loop {
        let (client_stream, client_addr) = listener.accept().await?;

        if check_rate_limit(&memcached, &client_addr.ip().to_string()).await? {
            println!("Client connected: {}", client_addr);

            tokio::spawn(async move {
                if let Err(e) = handle_connection(client_stream, client_addr).await {
                    eprintln!("Error handling connection: {}", e);
                }
            });
        } else {
            println!("Rate limit exceeded for: {}", client_addr);
        }
    }
}

async fn check_rate_limit(memcached: &Client, ip: &str) -> Result<bool, Box<dyn Error>> {
    let key = format!("rate_limit:{}", ip);
    let count: u32 = memcached
        .get(&key)
        .unwrap_or(Some(0))
        .expect("request count");
    println!("Current count: {}", count);

    if count >= 10 {
        return Ok(false);
    }

    memcached.set(&key, count + 1, 10)?;
    Ok(true)
}

async fn handle_connection(
    mut client_stream: TcpStream,
    client_addr: std::net::SocketAddr,
) -> Result<(), Box<dyn Error>> {
    println!("Handling request from client: {}", client_addr);

    let mut server_stream = TcpStream::connect("localhost:8080").await?;
    println!("Connected to the gRPC server");

    let (client_to_server, server_to_client) =
        io::copy_bidirectional(&mut client_stream, &mut server_stream).await?;

    println!(
        "Forwarded {} bytes from client to server and {} bytes from server to client",
        client_to_server, server_to_client
    );

    client_stream.shutdown().await?;
    server_stream.shutdown().await?;
    Ok(())
}
