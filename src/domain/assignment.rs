use derive_builder::Builder;
use diesel::internal::derives::multiconnection::chrono::NaiveDateTime;
use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable, Selectable};
use gen_server::models::{Assignment, AssignmentCreate, AssignmentUpdate};
use o2o::o2o;
use serde_json::Value;

static ID_PREFIX: &str = "as";

#[derive(
    o2o,
    Debug,
    Eq,
    Identifiable,
    Associations,
    PartialEq,
    Queryable,
    Selectable,
    Insertable,
    AsChangeset,
    Builder,
)]
#[from_owned(AssignmentCreate)]
#[from_owned(AssignmentUpdate)]
#[owned_into(Assignment)]
#[diesel(table_name = crate::modules::db::schema::assignments)]
#[diesel(belongs_to(crate::domain::lesson::LessonEntity, foreign_key = lesson_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AssignmentEntity {
    #[builder(default = "crate::modules::utils::id_gen::generate_id(ID_PREFIX)")]
    #[from(crate::modules::utils::id_gen::generate_id(ID_PREFIX))]
    pub id: String,

    #[builder(default = "None")]
    #[into(~.map(|x| x.and_utc()))]
    #[from({None})]
    pub created_at: Option<NaiveDateTime>,

    #[builder(default = "None")]
    #[into(~.map(|x| x.and_utc()))]
    #[from({None})]
    pub updated_at: Option<NaiveDateTime>,

    #[builder(default = "None")]
    #[into(~.map(|x| serde_json::from_value(x).expect("Failed to deserialize meta")))]
    #[from(~.map(|x| serde_json::to_value(x).expect("Failed to deserialize meta")))]
    pub meta: Option<Value>,

    #[map(~.clone())]
    pub lesson_id: String,

    #[map(~.clone())]
    pub title: String,

    #[map(~.clone())]
    pub description: Option<String>,

    #[into(~.and_utc())]
    #[from(~.naive_utc())]
    pub due_date: NaiveDateTime,
}
