use crate::modules::api::handler::ApiService;
use axum::async_trait;
use axum::extract::Host;
use axum::http::Method;
use axum_extra::extract::CookieJar;
use gen_server::apis::comments::{
    Comments, CreateCommentResponse, GetCommentResponse, ListCommentsResponse,
};
use gen_server::models::{
    CommentCreate, GetCommentPathParams,
    ListCommentsQueryParams,
};

#[async_trait]
impl Comments for ApiService {
    async fn create_comment(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _body: CommentCreate,
    ) -> Result<CreateCommentResponse, ()> {
        todo!()
    }

    async fn get_comment(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _path_params: GetCommentPathParams,
    ) -> Result<GetCommentResponse, ()> {
        todo!()
    }

    async fn list_comments(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _query_params: ListCommentsQueryParams,
    ) -> Result<ListCommentsResponse, ()> {
        todo!()
    }
}
