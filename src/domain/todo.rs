use derive_builder::Builder;
use diesel::internal::derives::multiconnection::chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, Selectable};

#[derive(Debug, Queryable, Selectable, Insertable, Builder)]
#[diesel(table_name = crate::modules::db::schema::todos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TodoEntity {
    pub id: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub title: String,
    pub description: Option<String>,
}
