use sea_orm_migration::prelude::*;
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Images::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Images::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Images::ImageUrl)
                            .string()
                            .not_null()
                    )
                    .col(ColumnDef::new(Images::CreatedAt).date_time())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Images::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Images {
    #[sea_orm(iden = "images")]
    Table,
    #[sea_orm(iden = "id")]
    Id,
    #[sea_orm(iden = "image_url")]
    ImageUrl,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
}