use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::data::db::get_conn;


#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "movies")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,

    #[sea_orm(column_type="Text")]
    pub title: String,

    pub runtime: String,

    pub production_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Production,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Production => Entity::belongs_to(super::production::Entity)
                .from(Column::ProductionId)
                .to(super::production::Column::Id)
                .into(),
        }
    }
}

impl Related<super::production::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Production.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}


pub async fn get_movies() -> Vec<Model> {
    let conn = get_conn().await;
    
    Entity::find()
        .all(conn)
        .await
        .unwrap()
        // .map_err(|e| anyhow::anyhow!(e))
}