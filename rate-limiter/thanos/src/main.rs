use std::net::SocketAddr;

use anyhow::*;

use http_body_util::{Empty, Full};
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

// use hyper_util::client::legacy::Client;
// use hyper_util::rt::{TokioExecutor, TokioIo};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let https = hyper_rustls::HttpsConnectorBuilder::new()
        .with_native_roots()
        .expect("no native root CA certificates found")
        .https_only()
        .enable_http1()
        .build();
    // let client: Client<_, Empty<Bytes>> = Client::builder(TokioExecutor::new()).build(https);
    // let url = ("https://httpbin.org/json")
    //     .parse()
    //     .context("Parsing URL")?;
    // let res = client.get(url).await.context("Performing HTTPS request");
    // println!("{:?}", res);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await?;

    println!("Listening on http://{}", addr);
    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(hello))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}

async fn hello(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, anyhow::Error> {
    println!("some called me");
    Ok(Response::new(Full::new(Bytes::from("Hello, World dapo!"))))
}
