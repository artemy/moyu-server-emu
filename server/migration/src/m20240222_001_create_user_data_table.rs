use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserData::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserData::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserData::Imei).string().not_null())
                    .col(ColumnDef::new(UserData::UserId).string().not_null())
                    .col(ColumnDef::new(UserData::ServiceToken).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserData::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum UserData {
    Table,
    Id,
    Imei,
    UserId,
    ServiceToken,
}
