use config;
use configure::Configure;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use rocket;
use r2d2;
use askama::Template;

#[derive(Template)]
#[template(path = "base.html")]
struct Base;

#[derive(Template)]
#[template(path = "index.html")]
struct Index<'a> {
    name: &'a str,
    _parent: Base,
}

#[get("/")]
fn index() -> Index<'static> {
    Index { name: "meow", _parent: Base }
}

#[derive(Template)]
#[template(path = "t/new.html")]
struct TextNew { _parent: Base }

#[get("/t/new")]
fn t_new() -> TextNew {
    TextNew { _parent: Base }
}

pub fn start() -> rocket::Rocket {
    use_default_config!();
    let config = config::Config::generate().unwrap();
    let pool_manager = ConnectionManager::<PgConnection>::new(config.database_string());
    let pool: r2d2::Pool<ConnectionManager<PgConnection>> =
        r2d2::Pool::new(pool_manager).expect("Failed to create a database connection pool");

    let routes = routes![
        index,
        t_new,
    ];

    rocket::ignite()
        .mount("/", routes)
        .manage(pool)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::local::*;
    use rocket::http::Status;

    #[test]
    fn index_works() {
        let client = Client::new(start()).unwrap();
        let req = client.get("/");
        let mut res = req.dispatch();
        assert_eq!(res.status(), Status::Ok);;
        let body_string = res.body_string().unwrap();
        assert!(body_string.contains("meow"), body_string);
    }

    #[test]
    fn t_new_works() {
        let client = Client::new(start()).unwrap();
        let req = client.get("/t/new");
        let mut res = req.dispatch();
        assert_eq!(res.status(), Status::Ok);;
        let body_string = res.body_string().unwrap();
        assert!(body_string.contains("<form"), body_string);
    }
}
