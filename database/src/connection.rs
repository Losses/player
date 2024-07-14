use arroy::distances::Euclidean;
use arroy::Database as ArroyDatabase;
use heed::{Env, EnvOpenOptions};
use log::LevelFilter;
use sea_orm::DbErr;
use sea_orm::{ConnectOptions, Database};
use std::path::PathBuf;
use std::time::Duration;

use migration::Migrator;
use migration::MigratorTrait;

pub async fn connect_main_db(lib_path: &str) -> sea_orm::DatabaseConnection {
    let path: PathBuf = [lib_path, ".0.db"].iter().collect();
    let path_str = path.into_os_string().into_string().unwrap();
    let db_url = format!("sqlite:{}?mode=rwc", path_str);
    println!("{}", db_url);
    let mut opt = ConnectOptions::new(db_url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(LevelFilter::Info);

    let db = Database::connect(opt).await.unwrap();

    initialize_db(&db).await.unwrap();

    db
}

pub async fn initialize_db(conn: &sea_orm::DatabaseConnection) -> Result<(), DbErr> {
    Migrator::refresh(conn).await
}

const DB_SIZE: usize = 2 * 1024 * 1024 * 1024;

/// Initialize the recommendation database.
///
/// # Arguments
/// * `db_path` - The path to the database directory.
///
/// # Returns
/// * `Result<(Env, ArroyDatabase<Euclidean>), Box<dyn std::error::Error>>` - The database environment and the Arroy database.
pub fn connect_recommendation_db(
    lib_path: &str,
) -> Result<(Env, ArroyDatabase<Euclidean>), Box<dyn std::error::Error>> {
    let path: PathBuf = [lib_path, ".1.db"].iter().collect();
    let path_str = path.into_os_string().into_string().unwrap();
    let env = unsafe { EnvOpenOptions::new().map_size(DB_SIZE).open(path_str) }?;
    let mut wtxn = env.write_txn()?;
    let db: ArroyDatabase<Euclidean> = env.create_database(&mut wtxn, None)?;
    wtxn.commit()?;
    Ok((env, db))
}
