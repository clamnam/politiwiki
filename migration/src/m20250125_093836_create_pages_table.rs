use sea_orm_migration::prelude::*;    
    #[derive(DeriveMigrationName)]
    pub struct Migration;
    
    #[async_trait::async_trait]
    impl MigrationTrait for Migration {
        async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
            manager
                .create_table(
                    Table::create()
                        .table(Pages::Table)
                        .if_not_exists()
                        .col(
                            ColumnDef::new(Pages::Id)
                                .integer()
                                .not_null()
                                .auto_increment()
                                .primary_key(),
                        )

                        .col(ColumnDef::new(Pages::Title).string().not_null())
                        .col(ColumnDef::new(Pages::Category).integer())

                        .col(ColumnDef::new(Pages::CreatedAt).date_time())
                        .col(ColumnDef::new(Pages::UpdatedAt).date_time())
                        .col(ColumnDef::new(Pages::History).json())

                        .to_owned(),
                )
                .await?;

            manager
                .create_foreign_key(
                    ForeignKeyCreateStatement::new()
                        .name("fk_page_category")
                        .from(Pages::Table, Pages::Category) 
                        .to(Categories::Table, Categories::Id) // But it references the "categories" table
                        .to_owned(),
                )
                .await?;
    
            Ok(())
        }
    
        async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
            manager
                .drop_table(Table::drop().table(Pages::Table).to_owned())
                .await
        }
    }
    
    #[derive(DeriveIden)]
    enum Pages {
        #[sea_orm(iden = "pages")]
        Table,
        #[sea_orm(iden = "id")]
        Id,
        #[sea_orm(iden = "title")]
        Title,
        #[sea_orm(iden = "category")]
        Category,
        #[sea_orm(iden = "created_at")]
        CreatedAt,
        #[sea_orm(iden = "updated_at")]
        UpdatedAt,
        #[sea_orm(iden = "history")]
        History,
    }

    #[derive(Iden)]
    enum Categories {
        #[iden = "categories"] // This is the table name (plural)
        Table,
        Id,
    }


