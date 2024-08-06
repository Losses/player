//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "media_file_albums")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub media_file_id: i32,
    pub album_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::albums::Entity",
        from = "Column::AlbumId",
        to = "super::albums::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Albums,
    #[sea_orm(
        belongs_to = "super::media_files::Entity",
        from = "Column::MediaFileId",
        to = "super::media_files::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    MediaFiles,
}

impl Related<super::albums::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Albums.def()
    }
}

impl Related<super::media_files::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MediaFiles.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
    #[sea_orm(entity = "super::albums::Entity")]
    Albums,
    #[sea_orm(entity = "super::media_files::Entity")]
    MediaFiles,
}