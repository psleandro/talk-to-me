use std::{env, net::SocketAddr};
use axum::{Router, routing::get};
use dotenvy::dotenv;
use tokio::net::TcpListener;

async fn root() -> String {
    return String::from("Hello world!");
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = Router::new().route("/", get(root));

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse()
        .expect("Failed to parse PORT!");

    let socket_addr = SocketAddr::from(([127,0,0,1], port));
    println!("App listening on {}", socket_addr);

    let listener = TcpListener::bind(socket_addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

