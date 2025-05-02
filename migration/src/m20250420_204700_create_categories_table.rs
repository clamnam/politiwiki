use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {


        manager
        .create_table(
            Table::create()
                .table(Categories::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Categories::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )

                .col(ColumnDef::new(Categories::Name).string().unique_key().not_null())

                .to_owned(),
        )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Categories::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Categories {
    #[sea_orm(iden = "categories")]
    Table,
    #[sea_orm(iden = "id")]
    Id,
    #[sea_orm(iden = "name")]
    Name,
}
