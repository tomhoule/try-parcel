use config;
use configure::Configure;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use rocket;
use r2d2;
// use rocket_contrib::Template;
use askama::Template;

// #[get("/")]
// fn index() -> Template {
//     Template::render("index", json!({}))
// }

// #[get("/t/new")]
// fn t_new() -> Template {
//     Template::render("t/new", json!({}))
// }

#[derive(Template)]
#[template(path = "askama.html")]
struct HelloAskama<'a> {
    name: &'a str,
}

#[get("/")]
fn index() -> HelloAskama<'static> {
    HelloAskama { name: "meow"}
}

pub fn start() -> rocket::Rocket {
    use_default_config!();
    let config = config::Config::generate().unwrap();
    let pool_manager = ConnectionManager::<PgConnection>::new(config.database_string());
    let pool: r2d2::Pool<ConnectionManager<PgConnection>> =
        r2d2::Pool::new(pool_manager).expect("Failed to create a database connection pool");

    rocket::ignite()
        .mount("/", routes![index])
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
}
