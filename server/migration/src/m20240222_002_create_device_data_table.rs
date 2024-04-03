use crate::m20240222_001_create_user_data_table::UserData;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DeviceData::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DeviceData::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(DeviceData::Mac).string().not_null())
                    .col(ColumnDef::new(DeviceData::DeviceId).string().not_null())
                    .col(ColumnDef::new(DeviceData::DeviceSecret).string().not_null())
                    .col(ColumnDef::new(DeviceData::Ssid).string())
                    .col(ColumnDef::new(DeviceData::Version).string())
                    .col(ColumnDef::new(DeviceData::VerifyCode).string())
                    .col(
                        ColumnDef::new(DeviceData::Updated)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(DeviceData::LanguageFrom).string().not_null())
                    .col(ColumnDef::new(DeviceData::LanguageTo).string().not_null())
                    .col(ColumnDef::new(DeviceData::UserId).big_integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_device_data_user_id")
                            .from(DeviceData::Table, DeviceData::UserId)
                            .to(UserData::Table, UserData::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(DeviceData::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum DeviceData {
    Table,
    Id,
    DeviceId,
    DeviceSecret,
    Mac,
    Ssid,
    Version,
    VerifyCode,
    UserId,
    LanguageFrom,
    LanguageTo,
    Updated,
}
