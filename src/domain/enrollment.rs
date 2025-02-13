use derive_builder::Builder;
use diesel::internal::derives::multiconnection::chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use gen_server::models::{Enrollment, EnrollmentCreate, EnrollmentUpdate};
use o2o::o2o;
use serde_json::Value;
use std::str::FromStr;

static ID_PREFIX: &str = "er";

#[derive(o2o, Debug, Eq, PartialEq, Queryable, Selectable, Insertable, AsChangeset, Builder)]
#[from_owned(EnrollmentCreate)]
#[from_owned(EnrollmentUpdate)]
#[owned_into(Enrollment)]
#[diesel(table_name = crate::modules::db::schema::enrollments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EnrollmentEntity {
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
    pub user_id: String,

    #[map(~.clone())]
    pub course_id: String,

    #[into(gen_server::models::EnrollmentType::from_str(&~).unwrap())]
    #[from(~.to_string())]
    pub enrollment_type: String,
}
