use derive_builder::Builder;
use diesel::internal::derives::multiconnection::chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable, Selectable, Identifiable};
use serde_json::Value;

static ID_PREFIX: &str = "ct";

#[derive(Debug, Eq, PartialEq, Queryable, Identifiable, Selectable, Insertable, AsChangeset, Builder)]
#[diesel(table_name = crate::modules::db::schema::accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AccountEntity {
    #[builder(default = "crate::modules::utils::id_gen::generate_id(ID_PREFIX)")]
    pub id: String,

    #[builder(default = "None")]
    pub created_at: Option<NaiveDateTime>,

    #[builder(default = "None")]
    pub updated_at: Option<NaiveDateTime>,

    #[builder(default = "None")]
    pub meta: Option<Value>,

    pub sub: String,

    pub name: String,
}
