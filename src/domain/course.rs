use crate::domain::core::CoreEntity;
use crate::modules::utils::id_gen::generate_id;
use derive_builder::Builder;
use diesel::internal::derives::multiconnection::chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use gen_server::models::{Course, CourseCreate, CourseUpdate};
use serde_json::{from_value, Value};

static ID_PREFIX: &str = "co";

#[derive(Debug, Queryable, Selectable, Insertable, AsChangeset, Builder)]
#[diesel(table_name = crate::modules::db::schema::courses)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CourseEntity {
    #[builder(default = "generate_id(ID_PREFIX)")]
    pub id: String,

    #[builder(default = "None")]
    pub created_at: Option<NaiveDateTime>,

    #[builder(default = "None")]
    pub updated_at: Option<NaiveDateTime>,

    #[builder(default = "None")]
    pub meta: Option<Value>,

    pub name: String,
    pub slug: String,
    pub description: Option<String>,
}

impl CoreEntity for CourseEntity {
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

impl Into<Course> for CourseEntity {
    fn into(self) -> Course {
        Course {
            id: self.id,
            created_at: self.created_at.map(|x| x.and_utc()),
            updated_at: self.updated_at.map(|x| x.and_utc()),
            meta: self
                .meta
                .map(|x| from_value(x).expect("Failed to deserialize meta")),

            name: self.name,
            slug: self.slug,
            description: self.description,
        }
    }
}

impl From<CourseCreate> for CourseEntity {
    fn from(value: CourseCreate) -> Self {
        CourseEntityBuilder::default()
            .slug(value.slug)
            .name(value.name)
            .description(value.description)
            .meta(value.meta.map(|meta| serde_json::to_value(meta).expect("Failed to serialize meta")))
            .build()
            .unwrap()
    }
}

impl From<CourseUpdate> for CourseEntity {
    fn from(value: CourseUpdate) -> Self {
        CourseEntityBuilder::default()
            .slug(value.slug)
            .name(value.name)
            .description(value.description)
            .meta(value.meta.map(|meta| serde_json::to_value(meta).expect("Failed to serialize meta")))
            .build()
            .unwrap()
    }
}
