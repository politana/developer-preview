use axum::Router;
use tokio::runtime::Runtime;
use tower_http::services::{ServeDir, ServeFile};
use std::{net::SocketAddr, path::Path};

pub fn serve_debug_site(path: &Path) {
    let runtime = Runtime::new().expect("Cannot start web server");
    runtime.block_on(async {
        let fallback_path = path.join("_fallback.html");
        let service = ServeDir::new(path)
            .fallback(ServeFile::new(fallback_path));
        let router = Router::new().fallback_service(service);
        let port = 8080;
        let address = SocketAddr::from(([127, 0, 0, 1], port));
        println!("Serving locally: localhost:{}", port);
        let listener = tokio::net::TcpListener::bind(address).await
            .expect("Cannot start web server");
        axum::serve(listener, router).await
            .expect("Cannot start web server");
    });
}
