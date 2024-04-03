use crate::endpoints::common::ToStringWithoutDashes;
use crate::endpoints::errors::AppError;
use crate::endpoints::errors::AppError::{DatabaseError, UserNotFound};
use entity::user_data;
use entity::user_data::{Entity as UserDataEntity, Model as UserData};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, TryIntoModel,
};
use uuid::Uuid;

#[derive(Clone)]
pub struct UserService {
    db: DatabaseConnection,
}

impl UserService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn get_user_data_by_service_token(
        &self,
        service_token: &str,
    ) -> Result<UserData, AppError> {
        UserDataEntity::find()
            .filter(user_data::Column::ServiceToken.eq(service_token))
            .one(&self.db)
            .await
            .map_err(|e| DatabaseError(format!("Database error: [{}]", e)))?
            .ok_or(UserNotFound)
    }

    pub async fn get_or_create_user(&self, imei: &String) -> Result<UserData, AppError> {
        let user_data = self.get_user_data(imei).await?;
        match user_data {
            Some(ud) => Ok(ud),
            None => self.create_user_data(imei).await,
        }
    }

    async fn get_user_data(&self, imei: &String) -> Result<Option<UserData>, AppError> {
        UserDataEntity::find()
            .filter(user_data::Column::Imei.eq(imei))
            .one(&self.db)
            .await
            .map_err(|e| DatabaseError(format!("Database error: [{}]", e)))
    }

    async fn create_user_data(&self, imei: &String) -> Result<UserData, AppError> {
        let user_id = Uuid::new_v4().to_string_without_dashes();
        let service_token = Uuid::new_v4().to_string_without_dashes();

        let user_data = user_data::ActiveModel {
            imei: Set(imei.to_string()),
            user_id: Set(user_id),
            service_token: Set(service_token),
            ..Default::default()
        };

        user_data
            .insert(&self.db)
            .await
            .and_then(|m| m.try_into_model())
            .map_err(|e| DatabaseError(format!("Database error: {}", e)))
    }
}
