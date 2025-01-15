use std::fmt::Display;
use tracing::error;

pub fn app_panic<T: Into<String> + Display>(message: T) -> ! {
    let msg = format!("{}: {}", "Panic", message);
    error!("{}", msg);
    panic!("{}", msg);
}