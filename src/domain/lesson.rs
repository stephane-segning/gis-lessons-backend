use derive_builder::Builder;
use diesel::internal::derives::multiconnection::chrono::NaiveDateTime;
use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable, Selectable};
use gen_server::models::{Lesson, LessonCreate, LessonUpdate};
use o2o::o2o;
use serde_json::Value;

static ID_PREFIX: &str = "ls";

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
#[from_owned(LessonCreate)]
#[from_owned(LessonUpdate)]
#[owned_into(Lesson)]
#[diesel(table_name = crate::modules::db::schema::lessons)]
#[diesel(belongs_to(crate::domain::module::ModuleEntity, foreign_key = module_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct LessonEntity {
    #[builder(default = "crate::modules::utils::id_gen::generate_id(ID_PREFIX)")]
    #[from(crate::modules::utils::id_gen::generate_id(ID_PREFIX))]
    #[map(~.clone())]
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
    pub module_id: String,

    #[map(~.clone())]
    pub title: String,

    #[into(serde_json::from_value(~).expect("Failed to deserialize content"))]
    #[from(serde_json::to_value(~).expect("Failed to deserialize content"))]
    pub content: Value,

    #[map(~.clone())]
    pub description: Option<String>,
}
