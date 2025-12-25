mod app;
mod db;
mod models;
mod repositories;
mod routes;
mod services;

use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

    let pool = db::pool::create_pool(&database_url).await;

    let app = app::create_app(pool);

    axum::serve(
        tokio::net::TcpListener::bind("127.0.0.1:3000")
            .await
            .unwrap(),
        app,
    )
    .await
    .unwrap();
}
