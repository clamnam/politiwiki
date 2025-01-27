use axum::Extension;
use sea_orm::DatabaseConnection;

pub async fn get_content(Extension(database): Extension<DatabaseConnection>) -> String {
    if database.ping().await.is_ok() {
        "Connected".to_string()
    } else {
        "Not connected".to_string()
    }
}