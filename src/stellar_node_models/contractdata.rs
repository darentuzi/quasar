//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "contractdata")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
    pub contractid: String,
    #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
    pub key: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub r#type: i32,
    #[sea_orm(column_type = "Text")]
    pub ledgerentry: String,
    pub lastmodified: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
