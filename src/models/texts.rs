use rpc::yacchauyo as proto;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::convert::From;
use diesel::prelude::*;
use db_schema::*;

#[derive(Identifiable, Queryable, Debug, PartialEq)]
pub struct Text {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub authors: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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
        p.set_id(id.to_string());
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
    pub description: String,
}

#[derive(Identifiable, AsChangeset, PartialEq, Debug)]
#[table_name = "texts"]
pub struct TextPatch {
    pub id: Uuid,
    pub title: Option<String>,
    pub authors: Option<String>,
    pub description: Option<String>,
}

impl NewText {
    pub fn save(&self, conn: &PgConnection) -> QueryResult<Text> {
        use db_schema::texts::dsl::*;
        ::diesel::insert_into(texts).values(self).get_result(conn)
    }
}

impl TextPatch {
    fn is_empty(&self) -> bool {
        if let &TextPatch {
            id: _,
            title: None,
            authors: None,
            description: None,
        } = self
        {
            true
        } else {
            false
        }
    }

    pub fn save(&self, conn: &PgConnection) -> QueryResult<Text> {
        use db_schema::texts::dsl::*;
        // Empty changesets cause query builder errors
        if self.is_empty() {
            texts.find(self.id).get_result(conn)
        } else {
            ::diesel::update(self).set(self).get_result(conn)
        }
    }
}

fn undefault(s: String) -> Option<String> {
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

// TODO: convert to TryFrom when available
impl From<proto::Text> for NewText {
    fn from(mut proto: proto::Text) -> NewText {
        NewText {
            title: proto.take_title(),
            slug: proto.take_slug(),
            authors: proto.take_authors(),
            description: proto.take_description(),
        }
    }
}

impl From<proto::Text> for TextPatch {
    fn from(mut proto: proto::Text) -> TextPatch {
        TextPatch {
            id: proto.take_id().parse().expect("id for update"),
            title: undefault(proto.take_title()),
            authors: undefault(proto.take_authors()),
            description: undefault(proto.take_description()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;
    use test_utils::*;
    use diesel::dsl::count;

    #[test]
    pub fn new_text_from_proto() {
        let mut proto = proto::Text::new();
        proto.set_title("the_title".to_string());
        proto.set_slug("slaggu".to_string());
        proto.set_authors("me and my pizza".to_string());

        assert_eq!(
            NewText::from(proto),
            NewText {
                title: "the_title".to_string(),
                slug: "slaggu".to_string(),
                authors: "me and my pizza".to_string(),
                description: "".to_string(),
            }
        )
    }

    #[test]
    fn update_with_empty_proto_is_noop() {
        let conn = db_setup();
        conn.begin_test_transaction().ok();
        let mut proto = proto::Text::new();
        proto.set_title("something1".to_string());
        proto.set_slug("something2".to_string());
        proto.set_authors("something3".to_string());

        let new = NewText::from(proto).save(&conn).unwrap();
        let mut patch = proto::Text::new();
        patch.id = new.id.to_string();
        let updated = TextPatch::from(patch).save(&conn).unwrap();
        assert_eq!(new, updated);
    }

    #[test]
    fn update_does_not_create() {
        use db_schema::texts::dsl::*;

        let conn = db_setup();
        conn.begin_test_transaction().ok();
        let mut proto = proto::Text::new();
        proto.set_title("something1".to_string());
        proto.set_slug("something2".to_string());
        proto.set_authors("something3".to_string());
        let new = NewText::from(proto).save(&conn).unwrap();
        let mut patch = proto::Text::new();
        patch.id = format!("{}", new.id);
        patch.title = "something4".to_string();
        let updated = TextPatch::from(patch).save(&conn).unwrap();
        let old_count: i64 = texts
            .filter(title.eq("something1"))
            .select(count(id))
            .get_result(&conn)
            .unwrap();
        let new_count: i64 = texts
            .filter(title.eq("something4"))
            .select(count(id))
            .get_result(&conn)
            .unwrap();
        assert_eq!(old_count, 0);
        assert_eq!(new_count, 1);
        assert_eq!(updated.id, new.id);
        assert_eq!(updated.authors, new.authors);
    }

    #[test]
    pub fn new_text_save_and_update() {
        use db_schema::texts::dsl::*;
        use diesel::dsl::*;

        let conn = db_setup();
        let mut proto = proto::Text::new();
        proto.set_title("batmann1".to_string());
        proto.set_slug("batmann2".to_string());
        proto.set_authors("batmann3".to_string());

        let mut updated = proto.clone();
        let new = NewText::from(proto).save(&conn).unwrap();

        updated.set_id(format!("{}", new.id));
        updated.set_title("batmann4".to_string());

        TextPatch::from(updated).save(&conn).unwrap();

        let original_title_count: i64 = texts
            .filter(title.eq("batmann1"))
            .select(count(title))
            .first(&conn)
            .unwrap();

        let new_title_count: i64 = texts
            .filter(title.eq("batmann4"))
            .select(count(title))
            .first(&conn)
            .unwrap();

        assert_eq!(original_title_count, 0);
        assert_eq!(new_title_count, 1);
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
