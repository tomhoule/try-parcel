use diesel::data_types::PgTimestamp;
use db_schema::*;
use uuid::Uuid;

#[derive(Identifiable, Queryable, Debug)]
pub struct Fragment {
    id: Uuid,
    schema_path: String,
    text_id: Option<Uuid>,
    value: Option<String>,
    created_at: PgTimestamp,
}
