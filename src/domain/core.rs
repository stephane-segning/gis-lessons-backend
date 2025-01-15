use diesel::internal::derives::multiconnection::chrono::NaiveDateTime;
use serde_json::Value;

pub trait CoreEntity {
    fn id(&self) -> String;
    fn created_at(&self) -> NaiveDateTime;
    fn updated_at(&self) -> NaiveDateTime;
    fn meta(&self) -> Value;
}
