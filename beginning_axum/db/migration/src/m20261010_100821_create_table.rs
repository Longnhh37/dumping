use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Username,
    Password,
}

#[derive(DeriveIden)]
enum Products {
    Table,
    Id,
    Title,
    Price,
    CategoryName,
}

#[derive(DeriveIden)]
enum Categories {
    Table,
    Name,
}

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261010_100821_create_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Users::Username).string().not_null())
                    .col(ColumnDef::new(Users::Password).string().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Categories::Table)
                    .if_not_exists()
                    .col(string(Categories::Name).primary_key())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Products::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Products::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Products::Title).string().not_null())
                    .col(ColumnDef::new(Products::Price).integer().not_null())
                    .col(ColumnDef::new(Products::CategoryName).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_product_category")
                            .from(Products::Table, Products::CategoryName)
                            .to(Categories::Table, Categories::Name),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Products::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table(Categories::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}
