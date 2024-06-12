use redis::cluster::{ClusterClient, ClusterConnection};
use redis::{self, Commands};
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::io::{self, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let nodes = vec![
        "redis://redis-0:6379",
        "redis://redis-1:6379",
        "redis://redis-2:6379",
    ];

    let client = ClusterClient::new(nodes.clone())?;
    let mut rediscon = client.get_connection()?;

    // let ip_address = "192.168.1.1";
    // let now =
    // rediscon.zadd(ip_address, "now", "now")?;

    // let val: i32 = con.get("test")?;
    // println!("key1: {}", val);
    // con.set("test", val + 1)?;
    // time::sleep(Duration::from_secs(1)).await;
    // }

    let listener = TcpListener::bind("0.0.0.0:8081").await?;
    println!("Proxy listening on 0.0.0.0:8081");
    loop {
        let (client_stream, client_addr) = listener.accept().await?;
        println!("Client connected: {}", client_addr);

        match check_rate_limit(&mut rediscon, &client_addr.ip().to_string()).await {
            Ok(_val) => {
                tokio::spawn(async move {
                    if let Err(e) = handle_connection(client_stream, client_addr).await {
                        eprintln!("Error handling connection: {}", e);
                    }
                });
            }
            Err(e) => {
                println!("Error: {}, client: {}", e, client_addr);
            }
        }
    }
}

async fn check_rate_limit(redis: &mut ClusterConnection, ip: &str) -> Result<bool, Box<dyn Error>> {
    let key = format!("rate_limit:{}", ip);
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as isize;
    let count: u32 = match redis.zcount(ip, now, now.saturating_sub(10))? {
        Some(value) => value,
        None => 0,
    };
    println!("Current count: {}", count);

    if count >= 10 {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Rate limit exceeded",
        )) as Box<dyn Error>);
    }

    redis.zadd(&key, &key, now)?;
    Ok(true)
}

async fn handle_connection(
    mut client_stream: TcpStream,
    client_addr: std::net::SocketAddr,
) -> Result<(), Box<dyn Error>> {
    println!("Handling request from client: {}", client_addr);

    let mut server_stream = TcpStream::connect("http://checkout").await?;
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
