use config;
use diesel;
use r2d2;
use diesel::prelude::*;
use configure::Configure;
use rpc::yacchauyo::{Text, Texts, TextsQuery};
use models;
use error::Error;
use protobuf::RepeatedField;

#[derive(Clone)]
pub struct Server {
    pool: r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>,
}

fn fill_repeated<Proto: From<T>, T>(target: &mut RepeatedField<Proto>, existing: Vec<T>) {
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

    pub fn create_text(&self, req: Text) -> Result<Text, Error> {
        let conn = &*self.pool.get()?;
        Ok(models::texts::NewText::from(req).save(&conn)?.into())
    }

    pub fn patch_text(&self, req: Text) -> Result<Text, Error> {
        let conn = self.pool.get()?;
        Ok(models::texts::TextPatch::from(req).save(&conn)?.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_utils::*;
    use diesel::dsl::*;

    #[test]
    fn server_new_works() {
        setup();
        Server::new();
    }

    #[test]
    fn texts_index_works() {
        setup();
        let req = TextsQuery::new();
        Server::new().texts_index(req).unwrap();
    }

    #[test]
    fn create_text_works() {
        use db_schema::texts::dsl::*;

        let conn = db_setup();
        let mut req = Text::new();
        req.set_title("Meow".to_string());
        req.set_authors("Some cats".to_string());
        req.set_slug("thats_new".to_string());

        let res = Server::new().create_text(req).unwrap();

        assert_eq!(res.get_title(), "Meow");
        assert_eq!(res.get_authors(), "Some cats");
        assert_eq!(res.get_slug(), "thats_new");

        let new_uuid: ::uuid::Uuid = res.id.parse().unwrap();
        let in_db: QueryResult<models::texts::Text> = texts.filter(id.eq(new_uuid)).first(&conn);
        assert!(in_db.is_ok());
    }

    #[test]
    fn create_text_is_idempotent() {
        let server = Server::new();
        let mut proto = Text::new();
        proto.slug = "my slug".to_string();
        proto.title = "my title".to_string();

        server.create_text(proto.clone()).unwrap();
        match server.create_text(proto) {
            Err(Error::Db {
                inner:
                    ::diesel::result::Error::DatabaseError(
                        ::diesel::result::DatabaseErrorKind::UniqueViolation,
                        _,
                    ),
            }) => (),
            Ok(_) => panic!("created a text twice"),
            Err(err) => panic!("Unexpected error: {:?}", err),
        }
    }

    #[test]
    fn patch_text_works() {
        use db_schema::texts::dsl::*;

        let conn = db_setup();
        let server = Server::new();
        let mut req = Text::new();
        req.set_title("Meow".to_string());
        let res = server.create_text(req).unwrap();
        let new_uuid: ::uuid::Uuid = res.id.parse().unwrap();

        let mut patch = Text::new();
        patch.set_id(format!("{}", new_uuid));
        patch.set_title("Woof".to_string());

        let res = server.patch_text(patch).unwrap();
        assert_eq!(res.title, "Woof");

        let count: i64 = texts
            .filter(title.eq("Woof"))
            .select(count(id))
            .get_result(&conn)
            .unwrap();
        assert_eq!(count, 1);
    }
}
