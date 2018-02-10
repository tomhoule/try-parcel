use askama::Template;
use diesel::prelude::*;
use error::Error;
use models::schemas::Schema;
use models::fragments::Fragment;
use models::texts::*;
use rocket_contrib::Json;
use rocket::response::Redirect;
use uuid::Uuid;
use web::shared::*;

#[derive(Template)]
#[template(path = "t/edit.html")]
pub struct TextEdit {
    text: Text,
    schema: Schema,
    _parent: Base,
}

#[derive(Template)]
#[template(path = "t/show.html")]
pub struct TextShow {
    text: Text,
    fragments: Vec<Fragment>,
    _parent: Base,
}

#[get("/t/<path_id>")]
pub fn t_show(pool: DbPool, path_id: String) -> Result<TextShow, Error> {
    use db_schema::texts::dsl::*;
    use db_schema::schemas;
    let conn: &PgConnection = &*pool.inner().get()?;
    let join = texts
        .find(path_id.parse::<::uuid::Uuid>()?)
        .left_join(schemas::table);
    let (text, schema) = join.first::<(Text, Option<Schema>)>(conn)?;
    // let schema = if let Some(schema) = schema {
    //     schema
    // } else {
    //     Schema::for_text(&conn, text.id)?
    // };

    Ok(TextShow {
        text,
        fragments: Vec::new(),
        _parent: Base,
    })
}

#[get("/t/<path_id>/edit")]
pub fn t_edit(pool: DbPool, path_id: String) -> Result<TextEdit, Error> {
    use db_schema::texts::dsl::*;
    use db_schema::schemas;
    let conn: &PgConnection = &*pool.inner().get()?;
    let join = texts
        .find(path_id.parse::<::uuid::Uuid>()?)
        .left_join(schemas::table);
    let (text, schema) = join.first::<(Text, Option<Schema>)>(conn)?;
    let schema = if let Some(schema) = schema {
        schema
    } else {
        Schema::for_text(&conn, text.id)?
    };
    Ok(TextEdit {
        text,
        schema,
        _parent: Base,
    })
}

#[post("/t/<path_id>/delete")]
pub fn t_delete(pool: DbPool, path_id: String) -> Result<Redirect, Error> {
    use db_schema::texts::dsl::*;
    use diesel;
    let conn: &PgConnection = &*pool.inner().get()?;
    let path_uuid: Uuid = path_id.parse()?;
    diesel::delete(texts.filter(id.eq(path_uuid))).execute(conn)?;
    Ok(Redirect::to("/"))
}

#[derive(Template)]
#[template(path = "t/new.html")]
pub struct TextNew {
    _parent: Base,
}

#[get("/t/new")]
pub fn t_new() -> TextNew {
    TextNew { _parent: Base }
}

#[post("/t", data = "<form>")]
pub fn t_create(pool: DbPool, form: Json<NewText>) -> Result<Json<Text>, Error> {
    let conn: &PgConnection = &*pool.inner().get()?;
    Ok(Json(form.save(conn)?))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::local::*;
    use rocket::http::Status;
    use web::start::start;

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
        PgConnection::establish(&::std::env::var("YACCHAUYO_DATABASE_URL").expect("YACCHAUYO_DATABASE_URL is defined"))
            .expect("connected to database")
    }

    #[test]
    fn t_edit_works() {
        let conn = db_conn();
        let text = NewText {
            title: "yacchauyo_test_0000".to_string(),
            authors: "meh".to_string(),
            description: "".to_string(),
            slug: "ahah".to_string(),
        };
        let text = text.save(&conn).expect("saved text");
        let client = Client::new(start()).expect("started client");
        let req = client.get(format!("/t/{}/edit", text.id));
        let res = req.dispatch();
        assert_eq!(res.status(), Status::Ok);
    }

    #[test]
    fn t_edit_with_bad_id() {
        let client = Client::new(start()).unwrap();
        let req = client.get("/t/a2223e80-f14d-4346-ab5/edit");
        let res = req.dispatch();
        assert_eq!(res.status(), Status::NotFound);
    }

    #[test]
    fn t_edit_with_inexistent_id() {
        let client = Client::new(start()).unwrap();
        let req = client.get("/t/a2223e80-f14d-4346-ab4d-9da7a042bf45/edit");
        let res = req.dispatch();
        assert_eq!(res.status(), Status::NotFound);
    }

    #[test]
    fn t_show_works() {
        let conn = db_conn();
        let text = NewText {
            title: "yacchauyo_test_0000".to_string(),
            authors: "meh".to_string(),
            description: "".to_string(),
            slug: "slug_00008".to_string(),
        };
        let text = text.save(&conn).expect("saved text");
        let client = Client::new(start()).expect("started client");
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

    #[test]
    fn t_delete_works() {
        use db_schema::texts::dsl::*;

        let conn = db_conn();
        let text = NewText {
            title: "yacchauyo_test_0000".to_string(),
            authors: "meh".to_string(),
            description: "".to_string(),
            slug: "meh".to_string(),
        };
        let text = text.save(&conn).unwrap();
        let client = Client::new(start()).unwrap();
        let req = client.post(format!("/t/{}/delete", text.id));
        let res = req.dispatch();
        assert_eq!(res.status(), Status::SeeOther);

        let text: Vec<Text> = texts.find(text.id).limit(1).load(&conn).unwrap();
        assert!(text.get(0).is_none());
    }
}
