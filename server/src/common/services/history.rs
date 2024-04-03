use crate::endpoints::errors::AppError;
use crate::endpoints::errors::AppError::DatabaseError;
use entity::chat_record::{
    ActiveModel as ChatRecordModel, Column as ChatRecordColumn, Entity as ChatRecordEntity,
    Model as ChatRecord,
};
use entity::device_data::{Column as DeviceDataColumn, Entity as DeviceDataEntity};
use entity::user_data::{Column as UserDataColumn, Entity as UserDataEntity};
use migration::JoinType;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect,
};

#[derive(Clone)]
pub struct ChatHistoryService {
    db: DatabaseConnection,
}

impl ChatHistoryService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
    pub async fn get_chat_history_by_user_id(
        &self,
        user_id: i64,
        page: u64,
        size: u64,
    ) -> Result<PaginatedChatRecords, AppError> {
        let paginator = ChatRecordEntity::find()
            .join_rev(
                JoinType::LeftJoin,
                DeviceDataEntity::belongs_to(ChatRecordEntity)
                    .from(DeviceDataColumn::Id)
                    .to(ChatRecordColumn::DeviceId)
                    .into(),
            )
            .join_rev(
                JoinType::LeftJoin,
                UserDataEntity::belongs_to(DeviceDataEntity)
                    .from(UserDataColumn::Id)
                    .to(DeviceDataColumn::UserId)
                    .into(),
            )
            .filter(UserDataColumn::Id.eq(user_id))
            .order_by_desc(ChatRecordColumn::Created)
            .paginate(&self.db, size);

        let records = paginator
            .fetch_page(page - 1)
            .await
            .map_err(|e| DatabaseError(format!("Database error: [{}]", e)))?;

        let count = records.len() as u32;

        let x = paginator.num_items().await;
        let more = x
            .map(|r| r > count.into())
            .map_err(|e| DatabaseError(format!("Database error: [{}]", e)))?;

        Ok(PaginatedChatRecords {
            count,
            list: records,
            more,
        })
    }

    pub async fn save_chat_record(
        &self,
        from: &String,
        to: &String,
        from_text: &String,
        to_text: &String,
        device_id: i64,
        is_myself: bool,
    ) -> Result<(), AppError> {
        let chat_record = ChatRecordModel {
            created: Set(chrono::Utc::now().into()),
            from: Set(from.to_owned()),
            to: Set(to.to_owned()),
            from_text: Set(from_text.to_owned()),
            to_text: Set(to_text.to_owned()),
            is_myself: Set(is_myself),
            device_id: Set(device_id),
            ..Default::default()
        };

        chat_record
            .insert(&self.db)
            .await
            .map(|_| ())
            .map_err(|e| DatabaseError(format!("Database error: {}", e)))
    }
}

pub struct PaginatedChatRecords {
    pub count: u32,
    pub list: Vec<ChatRecord>,
    pub more: bool,
}
