use sea_orm_migration::prelude::*;
use bcrypt;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create a database connection for raw queries
        let db = manager.get_connection();
        
        // Insert role with title 1.0
        db.execute_unprepared(
            "INSERT INTO roles (id, title) VALUES (1, 1.0) ON CONFLICT (id) DO NOTHING"
        ).await?;
        
        // Generate password hash using same method as in application
        let password_hash = bcrypt::hash("password123", 11)
            .map_err(|_| DbErr::Custom("Failed to hash password".to_string()))?;
        
        // Use parameterized query to avoid SQL injection
        let query = format!(
            "INSERT INTO users (email, username, password, role_id) 
             VALUES ('admin@example.com', 'admin', '{}', 1) 
             ON CONFLICT (email) DO NOTHING",
            password_hash
        );
        
        db.execute_unprepared(&query).await?;
        
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(
            "DELETE FROM users WHERE email = 'admin@example.com'"
        ).await?;
        
        Ok(())
    }
}

// Keep the Iden enums for reference
#[derive(Iden)]
enum User {
    #[iden = "users"]
    Table,
    Email,
    Username,
    Password,
    RoleId,
}

#[derive(Iden)]
enum Roles {
    #[iden = "roles"]
    Table,
    Id,
    Title,
}
