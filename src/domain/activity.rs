use crate::modules::db::schema::activities::dsl::activities;
use derive_builder::Builder;
use diesel::internal::derives::multiconnection::chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable, QueryableByName, Selectable};
use diesel_repository_macro::RepositoryAsync;
use o2o::o2o;
use serde_json::Value;
use std::str::FromStr;

static ID_PREFIX: &str = "ac";

#[derive(
    o2o,
    Debug,
    Eq,
    PartialEq,
    Queryable,
    Selectable,
    Insertable,
    AsChangeset,
    Builder,
    QueryableByName,
    RepositoryAsync,
)]
#[owned_into(gen_server::models::Activity)]
#[diesel(table_name = crate::modules::db::schema::activities)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[repo_table_name("activities")]
#[id_type("String")]
#[repo_features(get_one, save_one, delete_one, find_page, fts_search)]
#[fts_fields(content, entity_type)]
pub struct ActivityEntity {
    #[builder(default = "crate::modules::utils::id_gen::generate_id(ID_PREFIX)")]
    #[from(crate::modules::utils::id_gen::generate_id(ID_PREFIX))]
    pub id: String,

    #[builder(default = "None")]
    #[map(~.map(|x| x.and_utc()))]
    #[from({None})]
    pub created_at: Option<NaiveDateTime>,

    #[builder(default = "None")]
    #[map(~.map(|x| x.and_utc()))]
    #[from({None})]
    pub updated_at: Option<NaiveDateTime>,

    #[builder(default = "None")]
    #[into(~.map(|x| serde_json::from_value(x).expect("Failed to deserialize meta")))]
    #[from(~.map(|x| serde_json::to_value(x).expect("Failed to deserialize meta")))]
    pub meta: Option<Value>,

    #[map(~.clone())]
    pub user_id: String,

    #[map(~.clone())]
    pub entity_id: String,

    #[map(gen_server::models::ActivityEntityType::from_str(&~).unwrap())]
    pub entity_type: String,

    #[map(~.clone())]
    pub content: String,

    #[map(gen_server::models::ActivityType::from_str(&~).unwrap())]
    pub action_type: String,
}
