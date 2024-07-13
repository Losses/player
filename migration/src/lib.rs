pub use sea_orm_migration::prelude::*;

mod m20230701_000001_create_media_files_table;
mod m20230701_000002_create_media_metadata_table;
mod m20230701_000003_create_media_analysis_table;
mod m20230701_000004_create_user_logs_table;
mod m20230701_000005_create_playlists_table;
mod m20230701_000006_create_playlist_items_table;
mod m20230701_000007_create_smart_playlists_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230701_000001_create_media_files_table::Migration),
            Box::new(m20230701_000002_create_media_metadata_table::Migration),
            Box::new(m20230701_000003_create_media_analysis_table::Migration),
            Box::new(m20230701_000004_create_user_logs_table::Migration),
            Box::new(m20230701_000005_create_playlists_table::Migration),
            Box::new(m20230701_000006_create_playlist_items_table::Migration),
            Box::new(m20230701_000007_create_smart_playlists_table::Migration),
        ]
    }
}