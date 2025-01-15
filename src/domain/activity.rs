use crate::domain::core::CoreEntity;
use crate::modules::utils::id_gen::generate_id;
use derive_builder::Builder;
use diesel::internal::derives::multiconnection::chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use gen_server::models::{Activity, ActivityEntityType, ActivityType};
use serde_json::{from_value, Value};
use std::str::FromStr;

static ID_PREFIX: &str = "ac";

#[derive(Debug, Queryable, Selectable, Insertable, AsChangeset, Builder)]
#[diesel(table_name = crate::modules::db::schema::activities)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ActivityEntity {
    #[builder(default = "generate_id(ID_PREFIX)")]
    pub id: String,

    #[builder(default = "None")]
    pub created_at: Option<NaiveDateTime>,
    
    #[builder(default = "None")]
    pub updated_at: Option<NaiveDateTime>,
    
    #[builder(default = "None")]
    pub meta: Option<Value>,

    pub user_id: String,
    pub entity_id: String,
    pub entity_type: String,
    pub content: String,
    pub action_type: String,
}

impl CoreEntity for ActivityEntity {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn created_at(&self) -> NaiveDateTime {
        self.created_at.unwrap()
    }

    fn updated_at(&self) -> NaiveDateTime {
        self.updated_at.unwrap()
    }

    fn meta(&self) -> Value {
        self.meta.clone().unwrap()
    }
}

impl Into<Activity> for ActivityEntity {
    fn into(self) -> Activity {
        Activity {
            id: self.id,
            created_at: self.created_at.map(|x| x.and_utc()),
            updated_at: self.updated_at.map(|x| x.and_utc()),
            meta: self
                .meta
                .map(|x| from_value(x).expect("Failed to deserialize meta")),
            user_id: self.user_id,
            entity_id: self.entity_id,
            entity_type: ActivityEntityType::from_str(&self.entity_type).unwrap(),
            content: self.content,
            action_type: ActivityType::from_str(&self.action_type).unwrap(),
        }
    }
}
