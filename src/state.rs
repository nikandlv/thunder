use crate::vendor::configuration::Configuration;
use r2d2_postgres::PostgresConnectionManager;

pub struct State {
    pub database :  r2d2::Pool<PostgresConnectionManager> ,
    pub configuration: Configuration
}
