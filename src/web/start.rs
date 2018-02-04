use config;
use configure::Configure;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use rocket;
use r2d2;

#[get("/")]
fn index() -> String {
    "oh, hi mark".to_string()
}

pub fn start() -> rocket::Rocket {
    use_default_config!();
    let config = config::Config::generate().unwrap();
    let pool_manager = ConnectionManager::<PgConnection>::new(config.database_string());
    let pool: r2d2::Pool<ConnectionManager<PgConnection>> =
        r2d2::Pool::new(pool_manager).expect("Failed to create a database connection pool");

    rocket::ignite()
        .mount("/", routes![index])
        // .attach(Template::fairing())
        .manage(pool)
}
