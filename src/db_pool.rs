use diesel::mysql::MysqlConnection;
use r2d2::{Pool, Config};
use r2d2_diesel::ConnectionManager;
use dotenv::dotenv;
use std::env;

pub type DbPool = Pool<ConnectionManager<MysqlConnection>>;

pub fn create_db_pool() -> DbPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let config = Config::default();
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    Pool::new(config, manager).expect("Failed to create pool.")
}
