//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "device_data")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub mac: String,
    pub device_id: String,
    pub device_secret: String,
    pub ssid: Option<String>,
    pub version: Option<String>,
    pub verify_code: Option<String>,
    pub updated: DateTimeWithTimeZone,
    pub language_from: String,
    pub language_to: String,
    pub user_id: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::chat_record::Entity")]
    ChatRecord,
    #[sea_orm(
        belongs_to = "super::user_data::Entity",
        from = "Column::UserId",
        to = "super::user_data::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    UserData,
}

impl Related<super::chat_record::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ChatRecord.def()
    }
}

impl Related<super::user_data::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserData.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
