mod program;

use std::io;
use axum::{serve, Router};
use clap::Parser;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use program::Args;

#[tokio::main]
async fn main() -> io::Result<()> {
    let args = Args::parse();
    let url = format!("http://{}", &args.addr);
    println!();
    println!("  Listening on {} .", &url);
    println!();
    println!("  You can press <Ctrl+C> to shutdown.");
    println!();
    let app = Router::new().nest_service("/", ServeDir::new(args.dir));
    let listener = TcpListener::bind(&args.addr).await?;
    if args.open {
        let _ = webbrowser::open(&url);
    }
    serve(listener, app).await
}
