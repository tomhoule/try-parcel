use diesel::prelude::*;
use dotenv;

pub fn db_setup() -> PgConnection {
    setup();
    let database_url = ::std::env::var("YACCHAUYO_DATABASE_URL").unwrap();
    PgConnection::establish(&database_url).unwrap()
}

pub fn setup() {
    dotenv::dotenv().ok();
    use_default_config!();
}
