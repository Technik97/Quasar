use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "productions")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,

    #[sea_orm(column_type="Text", unique)]
    pub name: String,

}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Movie,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Movie => Entity::has_many(super::movie::Entity).into(),
        }
    }
}

impl Related<super::movie::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Movie.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}