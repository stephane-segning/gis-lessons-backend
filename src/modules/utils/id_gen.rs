use cuid2::cuid;

pub fn generate_id<T : Into<String>>(r#type: T) -> String {
    let id = cuid();
    format!("{}_{}", r#type.into(), id)
}