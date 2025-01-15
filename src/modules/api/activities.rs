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
        query_params: GetSubmissionActivitiesQueryParams,
    ) -> Result<GetSubmissionActivitiesResponse, ()> {
        let page = self
            .activity_service
            .find_activities(query_params.limit, query_params.offset, query_params.q)
            .await
            .expect("Failed to find activities");

        Ok(GetSubmissionActivitiesResponse::Status200_TheRequestedSubmission(page))
    }
}
