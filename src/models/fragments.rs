use diesel::data_types::PgTimestamp;
use db_schema::*;
use uuid::Uuid;

/// Fragments are attached to texts, and occupy a specific slot in their schemas.
///
/// They are immutable and persisted. Only the latest fragment for a given path will be displayed.
///
/// When a fragment has a path that does not exist in the schema anymore, it is considered orphaned.
#[derive(Identifiable, Queryable, Debug)]
pub struct Fragment {
    /// The fragment's primary identifier.
    pub id: Uuid,
    /// The path of the fragment in the text's schema.
    pub schema_path: String,
    /// The text this fragment belongs to.
    pub text_id: Option<Uuid>,
    /// The markdown text of the fragment.
    pub value: Option<String>,
    /// The creation time of the fragment.
    ///
    /// This is important in that only the latest fragment for a given path and text should be considered.
    pub created_at: PgTimestamp,
}
