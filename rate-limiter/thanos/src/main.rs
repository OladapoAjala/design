use memcache::Client;
use std::error::Error;
use tokio::io::{self, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("0.0.0.0:8081").await?;
    println!("Proxy listening on 0.0.0.0:8081");

    let memcached =
        memcache::connect("memcache://cache-memcached:11211?timeout=10&tcp_nodelay=true")
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;
    memcached
        .flush()
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;

    loop {
        let (client_stream, client_addr) = listener.accept().await?;
        println!("Client connected: {}", client_addr);

        if check_rate_limit(&memcached, &client_addr.ip().to_string()).await? {
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
    let mut count: u32 = match memcached.get(&key)? {
        Some(value) => value,
        None => 0,
    };
    println!("Current count: {}", count);

    if count >= 10 {
        return Ok(false);
    }

    count += 1;
    memcached.set(&key, count, 10)?;
    Ok(true)
}

async fn handle_connection(
    mut client_stream: TcpStream,
    client_addr: std::net::SocketAddr,
) -> Result<(), Box<dyn Error>> {
    println!("Handling request from client: {}", client_addr);

    let mut server_stream = TcpStream::connect("127.0.0.1:3000").await?;
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
