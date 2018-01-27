use diesel::prelude::*;
use config::Config;
use configure::Configure;

pub fn db_setup() -> PgConnection {
    setup();
    let config = Config::generate().unwrap();
    let database_url = config.database_string();
    PgConnection::establish(&database_url).unwrap()
}

pub fn setup() {
    use_default_config!();
}
