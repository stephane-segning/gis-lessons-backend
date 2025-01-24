use derive_builder::Builder;
use diesel::internal::derives::multiconnection::chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use gen_server::models::{LessonBlock, LessonBlockUpdate};
use o2o::o2o;
use serde_json::Value;

static ID_PREFIX: &str = "lb";

#[derive(o2o, Debug, Eq, PartialEq, Queryable, Selectable, Insertable, AsChangeset, Builder)]
#[from_owned(LessonBlockUpdate)]
#[owned_into(LessonBlock)]
#[diesel(table_name = crate::modules::db::schema::lesson_blocks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct LessonBlockEntity {
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
    pub r#type: String,

    #[into(serde_json::from_value(~).expect("Failed to deserialize data"))]
    #[from(serde_json::to_value(~).expect("Failed to deserialize data"))]
    pub data: Value,
}
