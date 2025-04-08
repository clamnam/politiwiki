// filepath: /Users/jackmoloneyobrien/Desktop/College/Major Project/PolitiWiki/src/main.rs
use politiwiki::run;
use dotenvy::dotenv;
use std::env;

#[tokio::main]
pub async fn main() {
    dotenv().ok();
    // Get DATABASE_URL at runtime instead of compile time
    let database_uri = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    // For debugging (uncomment if needed)
    // println!("Using database: {}", database_uri);
    
    run(&database_uri).await;
}