pub use sea_orm_migration::prelude::*;

mod m20240222_001_create_user_data_table;
mod m20240222_002_create_device_data_table;
mod m20240222_003_create_chat_history_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240222_001_create_user_data_table::Migration),
            Box::new(m20240222_002_create_device_data_table::Migration),
            Box::new(m20240222_003_create_chat_history_table::Migration),
        ]
    }
}
