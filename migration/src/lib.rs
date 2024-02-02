pub use sea_orm_migration::prelude::*;

mod m20240202_124954_create_table_users;
mod m20240202_125007_create_table_todos;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240202_124954_create_table_users::Migration),
            Box::new(m20240202_125007_create_table_todos::Migration),
        ]
    }
}
