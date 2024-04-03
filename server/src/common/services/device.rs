use crate::endpoints::common::ToStringWithoutDashes;
use crate::endpoints::errors::AppError;
use crate::endpoints::errors::AppError::{DatabaseError, DeviceNotFound};
use entity::device_data;
use entity::device_data::{
    ActiveModel as DeviceDataModel, Entity as DeviceDataEntity, Model as DeviceData,
};
use rand::distr::{Distribution, Uniform};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    TryIntoModel,
};
use uuid::Uuid;

#[derive(Clone)]
pub struct DeviceService {
    pub db: DatabaseConnection,
}

impl DeviceService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn get_device_data_by_user_id(
        &self,
        user_id: i64,
    ) -> Result<Option<DeviceData>, AppError> {
        DeviceDataEntity::find()
            .filter(device_data::Column::UserId.eq(user_id))
            .one(&self.db)
            .await
            .map_err(|e| DatabaseError(format!("Database error: {}", e)))
    }

    pub async fn get_device_data_by_device_id(
        &self,
        device_id: &str,
    ) -> Result<DeviceData, AppError> {
        DeviceDataEntity::find()
            .filter(device_data::Column::DeviceId.eq(device_id))
            .one(&self.db)
            .await
            .map_err(|e| DatabaseError(format!("Database error: {}", e)))?
            .ok_or(DeviceNotFound)
    }

    pub async fn get_device_data_by_verify_code(
        &self,
        verify_code: &str,
    ) -> Result<DeviceData, AppError> {
        DeviceDataEntity::find()
            .filter(device_data::Column::VerifyCode.eq(verify_code))
            .filter(device_data::Column::UserId.is_null())
            .one(&self.db)
            .await
            .map_err(|e| DatabaseError(format!("Database error: {}", e)))?
            .ok_or(DeviceNotFound)
    }

    pub async fn get_or_create_device_data(&self, mac: &String) -> Result<DeviceData, AppError> {
        let device_registration = self.get_device_data_by_mac(mac).await?;
        match device_registration {
            Some(dr) => Ok(dr),
            None => self.create_device_data(mac).await,
        }
    }

    async fn get_device_data_by_mac(&self, mac: &String) -> Result<Option<DeviceData>, AppError> {
        DeviceDataEntity::find()
            .filter(device_data::Column::Mac.eq(mac))
            .one(&self.db)
            .await
            .map_err(|e| DatabaseError(format!("Database error: {}", e)))
    }

    async fn create_device_data(&self, mac: &String) -> Result<DeviceData, AppError> {
        let device_id = Uuid::new_v4().to_string_without_dashes();
        let device_secret = Uuid::new_v4().to_string_without_dashes();

        let device_data = DeviceDataModel {
            mac: Set(mac.into()),
            device_id: Set(device_id),
            device_secret: Set(device_secret),
            updated: Set(chrono::Utc::now().into()),
            language_from: Set("en".into()),
            language_to: Set("zh".into()),
            ..Default::default()
        };

        device_data
            .insert(&self.db)
            .await
            .and_then(|m| m.try_into_model())
            .map_err(|e| DatabaseError(format!("Database error: {}", e)))
    }

    async fn update_device_data<F>(
        &self,
        device_id: &str,
        update_function: F,
    ) -> Result<(), AppError>
    where
        F: Fn(DeviceData) -> DeviceDataModel,
    {
        let device_data = self.get_device_data_by_device_id(device_id).await?;
        update_function(device_data)
            .update(&self.db)
            .await
            .map(|_| ())
            .map_err(|err| DatabaseError(format!("Error: {}", err)))
    }

    pub async fn update_ssid(&self, device_id: &str, ssid: &String) -> Result<(), AppError> {
        self.update_device_data(device_id, |device_data: DeviceData| {
            let mut device_data_model = device_data.into_active_model();
            device_data_model.ssid = Set(Some(ssid.to_owned()));
            device_data_model.updated = Set(chrono::Utc::now().into());
            device_data_model
        })
        .await
    }

    pub async fn update_version(&self, device_id: &str, version: &String) -> Result<(), AppError> {
        self.update_device_data(device_id, |device_data: DeviceData| {
            let mut device_data_model = device_data.into_active_model();
            device_data_model.version = Set(Some(version.to_owned()));
            device_data_model.updated = Set(chrono::Utc::now().into());
            device_data_model
        })
        .await
    }

    pub async fn update_language(
        &self,
        device_id: &str,
        from: &String,
        to: &String,
    ) -> Result<(), AppError> {
        self.update_device_data(device_id, |device_data: DeviceData| {
            let mut device_data_model = device_data.into_active_model();
            device_data_model.language_from = Set(from.to_owned());
            device_data_model.language_to = Set(to.to_owned());
            device_data_model.updated = Set(chrono::Utc::now().into());
            device_data_model
        })
        .await
    }

    pub async fn generate_and_return_verify_code(
        &self,
        device_id: &str,
    ) -> Result<String, AppError> {
        let verify_code = {
            let mut rng = rand::rng();
            let uniform = Uniform::new_inclusive(0, 9);
            let x: Vec<String> = (0..4)
                .map(|_| uniform.unwrap().sample(&mut rng))
                .map(|n| n.to_string())
                .collect();
            x.join("")
        };

        self.update_device_data(device_id, |device_data: DeviceData| {
            let mut device_data_model = device_data.into_active_model();
            device_data_model.user_id = Set(None);
            device_data_model.verify_code = Set(Some(verify_code.clone()));
            device_data_model
        })
        .await?;

        Ok(verify_code)
    }

    pub async fn bind_device_to_user(
        &self,
        user_id: i64,
        device_id: &String,
    ) -> Result<(), AppError> {
        let device_data = DeviceDataEntity::find()
            .filter(device_data::Column::DeviceId.eq(device_id))
            .filter(device_data::Column::VerifyCode.is_not_null())
            .filter(device_data::Column::UserId.is_null())
            .one(&self.db)
            .await
            .map_err(|e| DatabaseError(format!("Database error: [{}]", e)))?
            .ok_or(DeviceNotFound)?;

        let mut model = device_data.into_active_model();
        model.user_id = Set(Some(user_id));
        model.verify_code = Set(None);
        model
            .update(&self.db)
            .await
            .map(|_| ())
            .map_err(|err| DatabaseError(format!("Error: {}", err)))
    }

    pub async fn unbind_device_from_user(
        &self,
        user_id: i64,
        device_id: &String,
    ) -> Result<(), AppError> {
        let device_data = DeviceDataEntity::find()
            .filter(device_data::Column::DeviceId.eq(device_id))
            .filter(device_data::Column::UserId.eq(user_id))
            .one(&self.db)
            .await
            .map_err(|e| DatabaseError(format!("Database error: [{}]", e)))?
            .ok_or(DeviceNotFound)?;

        let mut model = device_data.into_active_model();
        model.user_id = Set(None);
        model
            .update(&self.db)
            .await
            .map(|_| ())
            .map_err(|err| DatabaseError(format!("Error: {}", err)))
    }
}
