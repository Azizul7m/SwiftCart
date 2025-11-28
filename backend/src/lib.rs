use std::env;
mod handlers;
mod models;
mod routes;

pub async fn run() {
    // Load environment variables from .env file
    let db_url = env::var("DATABASE_URL").unwrap_or("DATABASE_URL should be set".to_string());

    println!("Database Url: {}", db_url);

    // Initialize the database connection
    let listaner = tokio::net::TcpListener::bind("0.0.0.0:4000")
        .await
        .expect("unable to bind to port 4000");

    // Start the server
    axum::serve(listaner, routes::app().await)
        .await
        .expect("failed to start server");
    println!("Backend is running");
}
