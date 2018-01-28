use rpc::yacchauyo as proto;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::convert::From;
use diesel::prelude::*;
use utils::*;
use protobuf::RepeatedField;
use db_schema::*;
use models::texts::Text;

#[derive(Identifiable, Associations, Queryable, Debug, PartialEq)]
#[belongs_to(Text, foreign_key="text_id")]
pub struct Schema {
    pub id: Uuid,
    pub text_id: Uuid,
    pub paths: Vec<String>,
    pub created_at: DateTime<Utc>,
}

impl Schema {
    pub fn for_text(conn: &PgConnection, req_text_id: Uuid) -> QueryResult<Schema> {
        use db_schema::schemas::dsl::*;
        let existing: Option<Schema> = schemas
            .filter(text_id.eq(req_text_id))
            .order(created_at.desc())
            .first(conn)
            .optional()?;
        match existing {
            None => ::diesel::insert_into(schemas).values(
                text_id.eq(req_text_id)
                ).get_result(conn),
            Some(schema) => Ok(schema)
        }
    }
}

impl From<Schema> for proto::Schema {
    fn from(schema: Schema) -> proto::Schema {
        let Schema {
            id,
            text_id,
            paths,
            created_at,
        } = schema;
        let mut proto = proto::Schema::new();
        proto.set_id(format!("{}", id));
        proto.set_text_id(format!("{}", text_id));
        proto.set_paths(RepeatedField::from_vec(paths));
        proto.set_created_at(timestamp(created_at));
        proto
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_utils::*;
    use models::texts::NewText;

    #[test]
    fn proto_from_schema_works() {
        let default = proto::Schema::new();

        let converted: proto::Schema = Schema {
            id: Uuid::new_v4(),
            text_id: Uuid::new_v4(),
            paths: vec!(),
            created_at: Utc::now(),
        }.into();

        assert!(default != converted);
    }

    #[test]
    fn schema_for_text_creates_missing_schemas() {
        let conn = db_setup();
        conn.begin_test_transaction().unwrap();
        let text = NewText {
            title: "ab".to_string(),
            authors: "cd".to_string(),
            slug: "ef".to_string(),
            description: "gh".to_string(),
        }.save(&conn).unwrap();

        let first = Schema::for_text(&conn, text.id).unwrap();
        let second = Schema::for_text(&conn, text.id).unwrap();
        let third = Schema::for_text(&conn, text.id).unwrap();

        assert_eq!(first, second);
        assert_eq!(second, third);
    }
}
