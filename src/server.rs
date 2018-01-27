use config;
use diesel;
use r2d2;
use diesel::prelude::*;
use configure::Configure;

#[derive(Clone)]
pub struct Server {
    pool: r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>,
}

impl Server {
    pub fn new() -> Self {
        let config = config::Config::generate().unwrap();
        let manager = diesel::r2d2::ConnectionManager::new(config.database_string());
        let pool = r2d2::Pool::new(manager).expect("r2d2 pool");
        Server {
            pool,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv;

    #[test]
    fn server_new_works() {
        dotenv::dotenv().ok();
        use_default_config!();
        Server::new();
    }
}
