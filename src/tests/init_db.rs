use sea_orm::{DatabaseConnection, DbConn, Schema, ConnectionTrait, ExecResult, Statement};
use crate::database::sea_orm_active_enums::Status;

pub async fn init_test_db(db: &DatabaseConnection) -> Result<(), anyhow::Error> {
    // Create tables
    let schema = Schema::new(db.get_database_backend());
    
    // Create your tables here
    // For example:
    db.execute(Statement::from_string(
        db.get_database_backend(),
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL,
            email TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL,
            role_id INTEGER
        )"
    )).await?;
    
    // Insert test data
    db.execute(Statement::from_string(
        db.get_database_backend(),
        "INSERT INTO users (username, email, password, role_id) VALUES 
         ('testuser', 'test@example.com', '$2b$10$your_hashed_password', 2)"
    )).await?;
    
    // Add more tables and test data as needed
    
    Ok(())
}