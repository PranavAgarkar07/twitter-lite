mod app;
mod db;
mod models;
mod routes;

use dotenvy::dotenv;
use std::env;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

    let pool = db::create_pool(&database_url).await;

    let app = app::create_app(pool);

    //Start server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    tracing::info!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
