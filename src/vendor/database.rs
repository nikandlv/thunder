use r2d2_postgres::PostgresConnectionManager;
use r2d2_postgres::TlsMode;

pub fn build(url : &str) -> r2d2::Pool<PostgresConnectionManager> {
    let manager = PostgresConnectionManager::new(url,TlsMode::None)
        .expect("Unable to connect to database");
    r2d2::Pool::new(manager).expect("Unable to connect to database")
}