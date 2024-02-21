mod Todo;

use axum::extract::Query;
use tokio;
use axum::response::{Html, IntoResponse};
use axum::Router;
use axum::routing::get;
use serde::Deserialize;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {

    // RUST_LOG=trace cargo run
    tracing_subscriber::fmt::init();

    // statics file path.
    let serve_dir =
        ServeDir::new("static")
        .not_found_service(ServeFile::new("static/default.html"));

    // root path of web app.
    let app = Router::new()
        .route("/", get(default_handler))
        // QueryInput { foo: 24, bar: "escalate", z: Some(71) }
        .route("/q",get(query))
        .nest_service("/static",ServeDir::new("static"))
        .fallback_service(serve_dir)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:9000")
        .await
        .unwrap();
    tracing::info!("Axum is listening on: {}",listener.local_addr().unwrap());
    axum::serve(listener,app).await.unwrap();


    println!("Hello, world!");
}

#[derive(Debug,Deserialize)]
struct QueryInput{
    foo: i32,
    bar: String,
    z: Option<i32>
}

async fn default_handler() -> Html<&'static str> {
    Html("<h1>Hola Axum.</h1>")
}

async fn query(Query(params):Query<QueryInput>) -> impl IntoResponse{
    tracing::info!("get params: {:?}",params);
    Html("<h2>Hola,this is a get-method.</h2>")
}
