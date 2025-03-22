use sea_orm_migration::prelude::*;    
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Email).string().unique_key().not_null())
                    .col(ColumnDef::new(User::Username).string().unique_key().not_null())
                    .col(ColumnDef::new(User::Password).string().not_null())
                    .col(ColumnDef::new(User::RoleId).integer())
                    .col(ColumnDef::new(User::ImagesId).integer())
                    .col(ColumnDef::new(User::CreatedAt).date_time())
                    .col(ColumnDef::new(User::Token).string()) 
                    .to_owned(),
            )
            .await?;

        // Foreign key to roles
        manager
            .create_foreign_key(
                ForeignKeyCreateStatement::new()
                    .name("fk_user_role")
                    .from(User::Table, User::RoleId)
                    .to(Roles::Table, Roles::Id)
                    .to_owned(),
            )
            .await?;

        // Foreign key to images
        manager
            .create_foreign_key(
                ForeignKeyCreateStatement::new()
                    .name("fk_user_image")
                    .from(User::Table, User::ImagesId)
                    .to(Images::Table, Images::Id)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum User {
    #[iden = "users"]
    Table,
    Id,
    Email,
    Username,
    Password,
    RoleId,
    ImagesId,
    CreatedAt,
    Token, 
}

#[derive(Iden)]
enum Roles {
    Table,
    Id,
}

#[derive(Iden)]
enum Images {
    Table,
    Id,
}
