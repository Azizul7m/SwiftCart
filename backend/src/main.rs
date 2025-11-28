use backend::run;
use dotenvy::dotenv;
mod handlers;

// TODO: setup diesel database connection pool
// TODO: setup redis cache
// TODO: setup JWT authentication
#[tokio::main]
async fn main() {
    dotenv().ok();
    run().await;
    println!("Hello it's a E-commarce project for portfolio ")
}
