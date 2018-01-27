use rpc::yacchauyo as proto;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::convert::From;
use diesel::prelude::*;

#[derive(Queryable, Debug)]
pub struct Text {
    id: Uuid,
    title: String,
    slug: String,
    authors: String,
    description: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Text {
    pub fn index(conn: &PgConnection) -> QueryResult<Vec<Text>> {
        use db_schema::yacchauyo::texts::dsl::*;
        texts.load(conn)
    }
}

impl From<Text> for proto::Text {
    fn from(txt: Text) -> proto::Text {
        let mut p = proto::Text::new();
        p.set_id(format!("{}", txt.id));
        p.set_title(txt.title);
        p
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_to_proto() {
        let id = Uuid::new_v4();
        let title = "meow".to_string();
        let slug = "mw".to_string();
        let authors = "those are it".to_string();
        let description = "this is it".to_string();
        let created_at = Utc::now();
        let updated_at = Utc::now();
        let text = Text {
            id,
            title: title.clone(),
            slug,
            authors: authors.clone(),
            description,
            created_at,
            updated_at,
        };

        let proto: proto::Text = text.into();
        assert_eq!(proto.title, title);
        assert_eq!(proto.id, format!("{}", id));
    }
}
