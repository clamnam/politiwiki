use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Content::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Content::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Content::Title).string())
                    .col(ColumnDef::new(Content::ContentType).integer())
                    .col(ColumnDef::new(Content::ContentBody).text())
                    .col(ColumnDef::new(Content::ImagesId).integer())
                    .col(ColumnDef::new(Content::CreatedById).integer())
                    .col(ColumnDef::new(Content::ModifiedById).integer())
                    .col(ColumnDef::new(Content::Status).integer())
                    .col(ColumnDef::new(Content::OrderId).integer())
                    .col(ColumnDef::new(Content::IsHidden).boolean())
                    .col(ColumnDef::new(Content::IsDeleted).boolean())
                    .col(ColumnDef::new(Content::CreatedAt).date_time())
                    .col(ColumnDef::new(Content::UpdatedAt).date_time())
                    .to_owned(),
            )
            .await?;

        // images_id -> images.id
        manager
            .create_foreign_key(
                ForeignKeyCreateStatement::new()
                    .name("fk_content_images")
                    .from(Content::Table, Content::ImagesId)
                    .to(Images::Table, Images::Id)
                    .to_owned(),
            )
            .await?;

        // created_by_id -> users.id
        manager
            .create_foreign_key(
                ForeignKeyCreateStatement::new()
                    .name("fk_content_user_created")
                    .from(Content::Table, Content::CreatedById)
                    .to(User::Table, User::Id)
                    .to_owned(),
            )
            .await?;

        // modified_by_id -> users.id
        manager
            .create_foreign_key(
                ForeignKeyCreateStatement::new()
                    .name("fk_content_user_modified")
                    .from(Content::Table, Content::ModifiedById)
                    .to(User::Table, User::Id)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Content::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Content {
    #[sea_orm(iden = "content")]
    Table,
    #[sea_orm(iden = "id")]
    Id,
    #[sea_orm(iden = "title")]
    Title,
    #[sea_orm(iden = "content_type")]
    ContentType,
    #[sea_orm(iden = "content_body")]
    ContentBody,
    #[sea_orm(iden = "images_id")]
    ImagesId,
    #[sea_orm(iden = "created_by_id")]
    CreatedById,
    #[sea_orm(iden = "modified_by_id")]
    ModifiedById,
    #[sea_orm(iden = "status")]
    Status,
    #[sea_orm(iden = "order_id")]
    OrderId,
    #[sea_orm(iden = "is_hidden")]
    IsHidden,
    #[sea_orm(iden = "is_deleted")]
    IsDeleted,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
    #[sea_orm(iden = "updated_at")]
    UpdatedAt,
}

#[derive(Iden)]
enum User {
    #[iden = "users"]
    Table,
    Id,
}

#[derive(Iden)]
enum Images {
    #[iden = "images"]
    Table,
    Id,
}