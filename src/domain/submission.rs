use std::str::FromStr;

use derive_builder::Builder;
use diesel::internal::derives::multiconnection::chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use gen_server::models::{Submission, SubmissionCreate, SubmissionUpdate};
use o2o::o2o;
use serde_json::Value;

static ID_PREFIX: &str = "su";

#[derive(o2o, Debug, Eq, PartialEq, Queryable, Selectable, Insertable, AsChangeset, Builder)]
#[from_owned(SubmissionCreate)]
#[from_owned(SubmissionUpdate)]
#[owned_into(Submission)]
#[diesel(table_name = crate::modules::db::schema::submissions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SubmissionEntity {
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
    pub assignment_id: String,

    #[into(~.map(|x| x.and_utc()))]
    #[from(~.map(|x| x.naive_utc()))]
    pub date_submitted: Option<NaiveDateTime>,
    
    #[into(gen_server::models::SubmissionStatus::from_str(&~).unwrap())]
    #[from(~.to_string())]
    pub status: String,

    #[map(~.clone())]
    pub content: String,
}
