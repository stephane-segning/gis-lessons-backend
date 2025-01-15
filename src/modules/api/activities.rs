use crate::modules::api::handler::ApiService;
use axum::async_trait;
use axum::extract::Host;
use axum::http::Method;
use axum_extra::extract::CookieJar;
use gen_server::apis::activities::{Activities, GetSubmissionActivitiesResponse};
use gen_server::models::GetSubmissionActivitiesQueryParams;

#[async_trait]
impl Activities for ApiService {
    async fn get_submission_activities(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _query_params: GetSubmissionActivitiesQueryParams,
    ) -> Result<GetSubmissionActivitiesResponse, ()> {
        todo!()
    }
}