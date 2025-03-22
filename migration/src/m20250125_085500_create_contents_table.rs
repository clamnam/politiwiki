use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::sea_query::extension::postgres::Type;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    
        manager
            .create_type(
                Type::create()
                    .as_enum(Alias::new("status"))
                    .values([
                        StatusValues::Pending, 
                        StatusValues::Approved, 
                        StatusValues::Rejected, 
                        StatusValues::Published
                    ])
                    .to_owned(),
            )
            .await?;

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
                    .col(ColumnDef::new(Content::PageId).integer())
                    .col(
                        ColumnDef::new(Content::Status)
                            .enumeration(
                                Alias::new("status"), 
                                [
                                    StatusValues::Pending, 
                                    StatusValues::Approved, 
                                    StatusValues::Rejected, 
                                    StatusValues::Published
                                ]
                            )
                    )
                    .col(ColumnDef::new(Content::OrderId).integer())
                    .col(ColumnDef::new(Content::IsHidden).boolean())
                    .col(ColumnDef::new(Content::IsDeleted).boolean())
                    .col(ColumnDef::new(Content::CreatedAt).date_time())
                    .col(ColumnDef::new(Content::UpdatedAt).date_time())
                    .col(ColumnDef::new(Content::History).json())

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

        // page_id -> pages.id
        manager
            .create_foreign_key(
                ForeignKeyCreateStatement::new()
                    .name("fk_content_page")
                    .from(Content::Table, Content::PageId)
                    .to(Pages::Table, Pages::Id)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Content::Table).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(Alias::new("status")).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(Iden)]
enum Content {
    #[iden = "content"]
    Table,
    #[iden = "id"]
    Id,
    #[iden = "title"]
    Title,
    #[iden = "content_type"]
    ContentType,
    #[iden = "content_body"]
    ContentBody,
    #[iden = "images_id"]
    ImagesId,
    #[iden = "created_by_id"]
    CreatedById,
    #[iden = "modified_by_id"]
    ModifiedById,
    #[iden = "page_id"]
    PageId,
    #[iden = "status"]
    Status,
    #[iden = "order_id"]
    OrderId,
    #[iden = "is_hidden"]
    IsHidden,
    #[iden = "is_deleted"]
    IsDeleted,
    #[iden = "created_at"]
    CreatedAt,
    #[iden = "updated_at"]
    UpdatedAt,
    #[iden = "history"]
    History,
}

#[derive(Iden)]
enum StatusValues {
    #[iden = "pending"]
    Pending,
    #[iden = "approved"]
    Approved,
    #[iden = "rejected"]
    Rejected,
    #[iden = "published"]
    Published,
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

#[derive(Iden)]
enum Pages {
    #[iden = "pages"]
    Table,
    Id,
}