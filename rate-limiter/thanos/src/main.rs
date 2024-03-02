use std::error::Error;
use tokio::io::{self, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    println!("Proxy listening on 0.0.0.0:3000");

    loop {
        let (client_stream, client_addr) = listener.accept().await?;
        println!("Client connected: {}", client_addr);

        tokio::spawn(async move {
            if let Err(e) = handle_connection(client_stream).await {
                eprintln!("Error handling connection: {}", e);
            }
        });
    }
}

async fn handle_connection(mut client_stream: TcpStream) -> Result<(), Box<dyn Error>> {
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
