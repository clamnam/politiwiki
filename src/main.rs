use politiwiki::run;
use std::env;
#[tokio::main]
pub async fn main() {
    // Get DATABASE_URL at runtime instead of compile time
    let database_uri = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    run(&database_uri).await;
}
