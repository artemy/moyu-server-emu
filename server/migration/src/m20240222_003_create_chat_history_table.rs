use crate::m20240222_002_create_device_data_table::DeviceData;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ChatRecord::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ChatRecord::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ChatRecord::Created)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(ChatRecord::From).string().not_null())
                    .col(ColumnDef::new(ChatRecord::To).string().not_null())
                    .col(ColumnDef::new(ChatRecord::FromText).string().not_null())
                    .col(ColumnDef::new(ChatRecord::ToText).string().not_null())
                    .col(ColumnDef::new(ChatRecord::IsMyself).boolean().not_null())
                    .col(
                        ColumnDef::new(ChatRecord::DeviceId)
                            .big_integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_chat_record_device_id")
                            .from(ChatRecord::Table, ChatRecord::DeviceId)
                            .to(DeviceData::Table, DeviceData::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ChatRecord::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ChatRecord {
    Table,
    Id,
    Created,
    DeviceId,
    From,
    To,
    FromText,
    ToText,
    IsMyself,
}
