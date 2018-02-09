use config;
use configure::Configure;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use rocket;
use r2d2;
use askama::Template;
use rocket_contrib::Json;
use models::texts::{NewText, Text};
use rocket::State;
use error::*;
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;

type DbPool<'a> = State<'a, r2d2::Pool<ConnectionManager<PgConnection>>>;

#[post("/t", data = "<form>")]
fn t_create(pool: DbPool, form: Json<NewText>) -> Result<Json<Text>, Error> {
    let conn: &PgConnection = &*pool.inner().get()?;
    Ok(Json(form.save(conn)?))
}

#[derive(Template)]
#[template(path = "base.html")]
struct Base;

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

#[derive(Template)]
#[template(path = "t/show.html")]
struct TextShow {
    text: Text,
    _parent: Base,
}

#[get("/t/<path_id>")]
fn t_show(pool: DbPool, path_id: String) -> Result<TextShow, Error> {
    use db_schema::texts::dsl::*;
    let conn: &PgConnection = &*pool.inner().get()?;
    Ok(TextShow {
        text: texts
            .find(&path_id.parse::<::uuid::Uuid>()?)
            .first(conn)?,
        _parent: Base,
    })
}

#[derive(Template)]
#[template(path = "t/new.html")]
struct TextNew {
    _parent: Base,
}

#[get("/t/new")]
fn t_new() -> TextNew {
    TextNew { _parent: Base }
}

#[get("/static/<file..>")]
fn static_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

pub fn start() -> rocket::Rocket {
    use_default_config!();
    let config = config::Config::generate().unwrap();
    let pool_manager = ConnectionManager::<PgConnection>::new(config.database_string());
    let pool: r2d2::Pool<ConnectionManager<PgConnection>> =
        r2d2::Pool::new(pool_manager).expect("Failed to create a database connection pool");

    let routes = routes![index, t_new, t_create, static_files, t_show];

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

    #[test]
    fn t_new_works() {
        let client = Client::new(start()).unwrap();
        let req = client.get("/t/new");
        let mut res = req.dispatch();
        assert_eq!(res.status(), Status::Ok);;
        let body_string = res.body_string().unwrap();
        assert!(body_string.contains("data-text-new-form"), body_string);
    }

    #[test]
    fn t_create_success_case() {
        use serde_json::{from_str, to_vec};
        use rocket::http::ContentType;

        let client = Client::new(start()).unwrap();
        let mut req = client.post("/t");
        req.add_header("application/json".parse::<ContentType>().unwrap());
        let payload = json!({
            "title": "meow",
            "slug": "chu",
            "authors": "lalala",
            "description": "",
        });
        req.set_body(to_vec(&payload).unwrap());
        let mut res = req.dispatch();
        assert_eq!(res.status(), Status::Ok);
        let body_string = res.body_string().unwrap();
        assert!(from_str::<Text>(&body_string).is_ok());
    }

    #[test]
    fn t_create_failure_case() {
        use serde_json::to_vec;
        use rocket::http::ContentType;

        let client = Client::new(start()).unwrap();
        let mut req = client.post("/t");
        req.add_header("application/json".parse::<ContentType>().unwrap());
        let payload = json!({
            "title": "meow",
            "slug": 33,
            "authors": "lalala",
            "description": "",
        });
        req.set_body(to_vec(&payload).unwrap());
        let res = req.dispatch();
        assert_eq!(res.status(), Status::BadRequest);
    }

    fn db_conn() -> PgConnection {
        PgConnection::establish(&::std::env::var("YACCHAUYO_DATABASE_URL").expect("YACCHAUYO_DATABASE_URL is defined")).unwrap()
    }

    #[test]
    fn t_show_works() {
        let conn = db_conn();
        let text = NewText {
            title: "yacchauyo_test_0000".to_string(),
            authors: "meh".to_string(),
            description: "".to_string(),
            slug: "ahah".to_string(),
        };
        let text = text.save(&conn).unwrap();
        let client = Client::new(start()).unwrap();
        let req = client.get(format!("/t/{}", text.id));
        let res = req.dispatch();
        assert_eq!(res.status(), Status::Ok);
    }

    #[test]
    fn t_show_with_bad_id() {
        let client = Client::new(start()).unwrap();
        let req = client.get("/t/a2223e80-f14d-4346-ab5");
        let res = req.dispatch();
        assert_eq!(res.status(), Status::NotFound);
    }

    #[test]
    fn t_show_with_inexistent_id() {
        let client = Client::new(start()).unwrap();
        let req = client.get("/t/a2223e80-f14d-4346-ab4d-9da7a042bf45");
        let res = req.dispatch();
        assert_eq!(res.status(), Status::NotFound);
    }
}
