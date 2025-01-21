use derive_builder::Builder;
use diesel::internal::derives::multiconnection::chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use gen_server::models::{Course, CourseCreate, CourseUpdate};
use o2o::o2o;
use serde_json::Value;

static ID_PREFIX: &str = "co";

#[derive(o2o, Debug, Eq, PartialEq, Queryable, Selectable, Insertable, AsChangeset, Builder)]
#[from_owned(CourseCreate)]
#[from_owned(CourseUpdate)]
#[owned_into(Course)]
#[diesel(table_name = crate::modules::db::schema::courses)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CourseEntity {
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
    pub name: String,

    #[map(~.clone())]
    pub slug: String,

    #[map(~.clone())]
    pub description: Option<String>,
}
