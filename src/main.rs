use std::env;
use std::net::SocketAddr;
use std::path::PathBuf;

use anyhow::Result;
use axum::Router;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

use myrustjourney::ssg::build_site;

#[tokio::main]
async fn main() -> Result<()> {
    let command = env::args().nth(1).unwrap_or_else(|| "build".to_string());

    let content_dir = env::var("CONTENT_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("content"));
    let out_dir = env::var("OUT_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("dist"));
    let site_title = env::var("SITE_TITLE").unwrap_or_else(|_| "Apoorva Sahay".to_string());

    let count = build_site(&content_dir, &out_dir, &site_title)?;
    println!("Built {count} static pages into {}/", out_dir.display());

    if command == "serve" {
        let port: u16 = env::var("PORT")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(8080);
        let app = Router::new().fallback_service(ServeDir::new(&out_dir));
        let addr = SocketAddr::from(([127, 0, 0, 1], port));
        let listener = TcpListener::bind(addr).await?;
        println!("Previewing static files at http://{addr}/ (Ctrl+C to stop)");
        axum::serve(listener, app).await?;
    }

    Ok(())
}
