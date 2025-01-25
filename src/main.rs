
use dotenvy_macro::dotenv;
use politiwiki::run;
//gets env data
use dotenvy::dotenv;

#[tokio::main]

pub async fn main(){
    dotenv().ok();
    let database_uri = dotenv!("DATABASE_URL");
    print!("{:?}", database_uri);
    run(&database_uri).await;
}