use askama::Template;
use config;
use configure::Configure;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use error::*;
use models::texts::Text;
use r2d2;
use rocket;
use rocket::response::NamedFile;
use std::path::{Path, PathBuf};
use web::shared::*;
use web::texts::*;

#[derive(Template)]
#[template(path = "index.html")]
struct Index<'a> {
    name: &'a str,
    texts: Vec<Text>,
    _parent: Base,
}

#[get("/")]
fn index(pool: DbPool) -> Result<Index<'static>, Error> {
    let conn: &PgConnection = &*pool.inner().get()?;
    Ok(Index {
        name: "meow",
        texts: Text::index(conn)?,
        _parent: Base,
    })
}

#[get("/static/<file..>")]
fn static_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[get("/docs/<file..>")]
fn docs(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("target/doc/").join(file)).ok()
}

pub fn start() -> rocket::Rocket {
    use_default_config!();
    let config = config::Config::generate().unwrap();
    let pool_manager = ConnectionManager::<PgConnection>::new(config.database_string());
    let pool: r2d2::Pool<ConnectionManager<PgConnection>> =
        r2d2::Pool::new(pool_manager).expect("Failed to create a database connection pool");

    let routes = routes![
        index,
        static_files,
        docs,
        t_create,
        t_delete,
        t_edit,
        t_new,
        t_show,
    ];

    rocket::ignite().mount("/", routes).manage(pool)
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
