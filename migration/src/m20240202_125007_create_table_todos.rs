use sea_orm_migration::prelude::*;
use crate::m20240202_124954_create_table_users::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Todos::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Todos::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Todos::Title).string().not_null())
                    .col(ColumnDef::new(Todos::Description).string().not_null())
                    .col(ColumnDef::new(Todos::Done).boolean().not_null().default(false))
                    .col(ColumnDef::new(Todos::CreatedBy).integer().not_null())
                    .col(ColumnDef::new(Todos::UpdatedBy).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Todos::Table, Todos::CreatedBy)
                            .to(Users::Table, Users::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Todos::Table, Todos::UpdatedBy)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Todos::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Todos {
    Table,
    Id,
    Title,
    Description,
    Done,
    CreatedBy,
    UpdatedBy,
}