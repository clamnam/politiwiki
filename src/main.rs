// filepath: /Users/jackmoloneyobrien/Desktop/College/Major Project/PolitiWiki/src/main.rs
use dotenvy_macro::dotenv;
use politiwiki::run;
use dotenvy::dotenv;

#[tokio::main]
pub async fn main() {
    dotenv().ok();
    let database_uri = dotenv!("DATABASE_URL");
    println!("{:?}", database_uri);
    run(&database_uri).await;
}