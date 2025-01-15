use derive_builder::Builder;
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::AsyncPgConnection;

#[derive(Clone, Builder)]
pub struct ApiService {
    pool: Pool<AsyncPgConnection>,
}

impl AsRef<ApiService> for ApiService {
    fn as_ref(&self) -> &ApiService {
        self
    }
}
