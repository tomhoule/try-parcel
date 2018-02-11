use diesel::data_types::PgTimestamp;
use db_schema::*;
use uuid::Uuid;

#[derive(Identifiable, Queryable, Debug)]
pub struct Fragment {
    pub id: Uuid,
    pub schema_path: String,
    pub text_id: Option<Uuid>,
    pub value: Option<String>,
    pub created_at: PgTimestamp,
}
