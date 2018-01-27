use rpc::yacchauyo as proto;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::convert::From;
use diesel::prelude::*;
use db_schema::*;

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
        use db_schema::texts::dsl::*;
        texts.load(conn)
    }
}

impl From<Text> for proto::Text {
    fn from(txt: Text) -> proto::Text {
        let Text {
            id,
            title,
            authors,
            description,
            slug,
            ..
        } = txt;
        let mut p = proto::Text::new();
        p.set_id(format!("{}", id));
        p.set_title(title);
        p.set_authors(authors);
        p.set_description(description);
        p.set_slug(slug);
        p
    }
}

#[derive(Insertable, PartialEq, Debug)]
#[table_name = "texts"]
pub struct NewText {
    pub title: String,
    pub slug: String,
    pub authors: String,
    pub description: Option<String>,
}

impl NewText {
    pub fn save(&self, conn: &PgConnection) -> QueryResult<Text> {
        use db_schema::texts::dsl::*;
        ::diesel::insert_into(texts)
            .values(self)
            .get_result(conn)
    }
}

impl From<proto::Text> for NewText {
    fn from(mut proto: proto::Text) -> NewText {
        NewText {
            title: proto.take_title(),
            slug: proto.take_slug(),
            authors: proto.take_authors(),
            description: Some(proto.take_description()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    pub fn new_text_from_proto() {
        let mut proto = proto::Text::new();
        proto.set_title("taïteul".to_string());
        proto.set_slug("slaggu".to_string());
        proto.set_authors("me and my pizza".to_string());

        assert_eq!(NewText::from(proto), NewText {
            title: "taïteul".to_string(),
            slug: "slaggu".to_string(),
            authors: "me and my pizza".to_string(),
            description: Some("".to_string()),
        })
    }

    #[test]
    fn text_to_proto() {
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
