use diesel::result::QueryResult;
use diesel_async::AsyncPgConnection;
use std::marker::Send;
use diesel::QueryableByName;
use diesel::sql_types::BigInt;

#[derive(Debug)]
pub struct Paged<T> {
    pub items: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub size: i64,
}

#[derive(QueryableByName, Debug)]
pub struct Count {
    #[sql_type = "BigInt"]
    pub count: i64,
}

pub trait GetOneByRepoAsync<T, ID>: Send + Sync {
    /// Asynchronously find a single entity by ID
    async fn get_one_by_id(conn: &mut AsyncPgConnection, id: ID) -> QueryResult<T>;
}

pub trait SaveOneRepoAsync<T>: Send + Sync {
    /// Asynchronously saves (inserts/upserts) a single entity.
    /// `data: impl Into<T>` allows using a custom "input" struct.
    async fn save_one(conn: &mut AsyncPgConnection, data: impl Into<T> + Send) -> QueryResult<T>;
}

pub trait DeleteOneRepoAsync<ID>: Send + Sync {
    async fn delete_one(conn: &mut AsyncPgConnection, id: ID) -> QueryResult<usize>;
}

pub trait FindPageRepoAsync<T>: Send + Sync {
    /// Return a page of items + total count, given `page` and `size`.
    async fn find_page(conn: &mut AsyncPgConnection, page: i64, size: i64)
                       -> QueryResult<Paged<T>>;
}

pub trait FullTextSearchRepoAsync<T>: Send + Sync {
    /// Perform a full-text search, returning a page of results.
    async fn full_text_search(
        conn: &mut AsyncPgConnection,
        query: &str,
        page: i64,
        size: i64,
    ) -> QueryResult<Paged<T>>;
}
