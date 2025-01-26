use std::str::FromStr;

use derive_builder::Builder;
use diesel::internal::derives::multiconnection::chrono::NaiveDateTime;
use diesel::{AsChangeset, Associations, Insertable, Queryable, Selectable};
use gen_server::models::{SubmissionMember, SubmissionMemberCreate, SubmissionMemberUpdate};
use o2o::o2o;
use serde_json::Value;

#[derive(
    o2o, Debug, Eq, Associations, PartialEq, Queryable, Selectable, Insertable, AsChangeset, Builder,
)]
#[from_owned(SubmissionMemberCreate)]
#[from_owned(SubmissionMemberUpdate)]
#[owned_into(SubmissionMember)]
#[diesel(table_name = crate::modules::db::schema::submission_members)]
#[diesel(belongs_to(crate::domain::assignment::AssignmentEntity, foreign_key = assignment_id))]
#[diesel(belongs_to(crate::domain::enrollment::EnrollmentEntity, foreign_key = enrollment_id))]
#[diesel(belongs_to(crate::domain::submission::SubmissionEntity, foreign_key = submission_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SubmissionMemberEntity {
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

    #[map(~.clone())]
    pub enrollment_id: String,

    #[map(~.clone())]
    pub submission_id: String,

    #[into(gen_server::models::SubmissionMemberRole::from_str(&~).unwrap())]
    #[from(~.to_string())]
    pub role: String,
}
