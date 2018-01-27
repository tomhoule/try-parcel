use config;
use diesel;
use r2d2;
use diesel::prelude::*;
use configure::Configure;
use rpc::yacchauyo::{Text, Texts, TextsQuery};
use error::Error;

#[derive(Clone)]
pub struct Server {
    pool: r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>,
}

impl Server {
    pub fn new() -> Self {
        let config = config::Config::generate().unwrap();
        let manager = diesel::r2d2::ConnectionManager::new(config.database_string());
        let pool = r2d2::Pool::new(manager).expect("r2d2 pool");
        Server { pool }
    }

    pub fn texts_index(&self, req: TextsQuery) -> Result<Texts, Error> {
        let mut response = ::rpc::yacchauyo::Texts::new();
        let mut first_text = ::rpc::yacchauyo::Text::new();
        first_text.set_title("Ethica more geometrico demonstrata".to_string());
        response.texts.push(first_text);
        Ok(response)
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
