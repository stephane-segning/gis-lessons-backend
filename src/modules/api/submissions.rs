use crate::modules::api::handler::ApiService;
use axum::async_trait;
use axum::extract::Host;
use axum::http::Method;
use axum_extra::extract::CookieJar;
use gen_server::apis::submissions::{
    CreateSubmissionResponse, DeleteSubmissionResponse, GetSubmissionResponse,
    ListSubmissionsResponse, Submissions, UpdateSubmissionResponse,
};
use gen_server::models::{
    DeleteSubmissionPathParams, GetSubmissionPathParams,
    ListSubmissionsQueryParams, SubmissionCreate, SubmissionUpdate, UpdateSubmissionPathParams,
};

#[async_trait]
impl Submissions for ApiService {
    async fn create_submission(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _body: SubmissionCreate,
    ) -> Result<CreateSubmissionResponse, ()> {
        todo!()
    }

    async fn delete_submission(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _path_params: DeleteSubmissionPathParams,
    ) -> Result<DeleteSubmissionResponse, ()> {
        todo!()
    }

    async fn get_submission(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _path_params: GetSubmissionPathParams,
    ) -> Result<GetSubmissionResponse, ()> {
        todo!()
    }

    async fn list_submissions(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _query_params: ListSubmissionsQueryParams,
    ) -> Result<ListSubmissionsResponse, ()> {
        todo!()
    }

    async fn update_submission(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _path_params: UpdateSubmissionPathParams,
        _body: SubmissionUpdate,
    ) -> Result<UpdateSubmissionResponse, ()> {
        todo!()
    }
}
