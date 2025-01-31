use crate::modules::api::handler::ApiService;
use axum::async_trait;
use axum::extract::Host;
use axum::http::Method;
use axum_extra::extract::CookieJar;
use gen_server::apis::comments::{
    Comments, CreateCommentResponse, GetCommentResponse, ListCommentsResponse,
};
use gen_server::models::{CommentCreate, GetCommentPathParams, ListCommentsQueryParams};

#[async_trait]
impl Comments for ApiService {
    async fn create_comment(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        body: CommentCreate,
    ) -> Result<CreateCommentResponse, ()> {
        let res = self
            .comment_service
            .create_entity(body)
            .await
            .expect("Failed to create comment");

        Ok(CreateCommentResponse::Status201_SubmissionCreatedSuccessfully(res))
    }

    async fn get_comment(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: GetCommentPathParams,
    ) -> Result<GetCommentResponse, ()> {
        let res = self
            .comment_service
            .get_entity(path_params.comment_id)
            .await
            .expect("Failed to get comment");

        Ok(GetCommentResponse::Status200_TheRequestedComment(res))
    }

    async fn list_comments(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        query_params: ListCommentsQueryParams,
    ) -> Result<ListCommentsResponse, ()> {
        let res = self
            .comment_service
            .find_entity(query_params.limit, query_params.offset, query_params.q)
            .await
            .expect("Failed to find comment");

        Ok(ListCommentsResponse::Status200_AListOfComments(res))
    }
}
