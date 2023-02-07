use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_user_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Create the User table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::FirstName).string().not_null())
                    .col(ColumnDef::new(User::LastName).string().not_null())
                    .col(ColumnDef::new(User::Age).integer().not_null())
                    .col(ColumnDef::new(User::IsCool).boolean().not_null())
                    .to_owned(),
            )
            .await
    }

    // Define how to rollback this migration: Drop the Bakery table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum User {
    Table,
    Id,
    FirstName,
    LastName,
    Age,
    IsCool,
}
