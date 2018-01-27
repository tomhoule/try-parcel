use config;
use diesel;
use r2d2;
use diesel::prelude::*;
use configure::Configure;
use rpc::yacchauyo::{
    Text,
    Texts,
    TextsQuery
};
use models;
use error::Error;
use protobuf::RepeatedField;

#[derive(Clone)]
pub struct Server {
    pool: r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>,
}

fn fill_repeated<
    Proto: From<T>,
    T,
>(target: &mut RepeatedField<Proto>, existing: Vec<T>) {
    for element in existing.into_iter() {
        target.push(element.into())
    }
}

impl Server {
    pub fn new() -> Self {
        let config = config::Config::generate().unwrap();
        let manager = diesel::r2d2::ConnectionManager::new(config.database_string());
        let pool = r2d2::Pool::new(manager).expect("r2d2 pool");
        Server { pool }
    }

    pub fn texts_index(&self, _req: TextsQuery) -> Result<Texts, Error> {
        let conn = &*self.pool.get()?;
        let texts = models::texts::Text::index(conn)?;
        let mut response = ::rpc::yacchauyo::Texts::new();
        fill_repeated(&mut response.texts, texts);      
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
