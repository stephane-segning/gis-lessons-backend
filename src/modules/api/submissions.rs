use crate::modules::api::handler::ApiService;
use axum::async_trait;
use axum::extract::Host;
use axum::http::Method;
use axum_extra::extract::CookieJar;
use gen_server::apis::submissions::{
    CreateSubmissionMemberResponse, CreateSubmissionResponse, DeleteSubmissionMemberResponse,
    DeleteSubmissionResponse, GetSubmissionMemberResponse, GetSubmissionResponse,
    ListSubmissionMembersResponse, ListSubmissionsResponse, Submissions,
    UpdateSubmissionMemberResponse, UpdateSubmissionResponse,
};
use gen_server::models::{
    CreateSubmissionMemberPathParams, DeleteSubmissionMemberPathParams, DeleteSubmissionPathParams,
    GetSubmissionMemberPathParams, GetSubmissionPathParams, ListSubmissionMembersPathParams,
    ListSubmissionsQueryParams, SubmissionCreate, SubmissionMemberCreate, SubmissionMemberUpdate,
    SubmissionUpdate, UpdateSubmissionMemberPathParams, UpdateSubmissionPathParams,
};

#[async_trait]
impl Submissions for ApiService {
    async fn create_submission(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        body: SubmissionCreate,
    ) -> Result<CreateSubmissionResponse, ()> {
        let res = self
            .submission_service
            .create_entity(body)
            .await
            .expect("Failed to create submission");

        Ok(CreateSubmissionResponse::Status201_SubmissionCreatedSuccessfully(res))
    }

    async fn create_submission_member(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: CreateSubmissionMemberPathParams,
        body: SubmissionMemberCreate,
    ) -> Result<CreateSubmissionMemberResponse, ()> {
        todo!()
    }

    async fn delete_submission(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: DeleteSubmissionPathParams,
    ) -> Result<DeleteSubmissionResponse, ()> {
        self.submission_service
            .delete_entity(path_params.submission_id)
            .await
            .expect("Failed to delete submission");

        Ok(DeleteSubmissionResponse::Status204_SubmissionDeletedSuccessfully)
    }

    async fn delete_submission_member(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: DeleteSubmissionMemberPathParams,
    ) -> Result<DeleteSubmissionMemberResponse, ()> {
        todo!()
    }

    async fn get_submission(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: GetSubmissionPathParams,
    ) -> Result<GetSubmissionResponse, ()> {
        let res = self
            .submission_service
            .get_entity(path_params.submission_id)
            .await
            .expect("Failed to get submission");

        Ok(GetSubmissionResponse::Status200_TheRequestedSubmission(res))
    }

    async fn get_submission_member(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: GetSubmissionMemberPathParams,
    ) -> Result<GetSubmissionMemberResponse, ()> {
        todo!()
    }

    async fn list_submission_members(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: ListSubmissionMembersPathParams,
    ) -> Result<ListSubmissionMembersResponse, ()> {
        todo!()
    }

    async fn list_submissions(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        query_params: ListSubmissionsQueryParams,
    ) -> Result<ListSubmissionsResponse, ()> {
        let res = self
            .submission_service
            .find_entity(query_params.limit, query_params.offset, query_params.q)
            .await
            .expect("Failed to find submission");

        Ok(ListSubmissionsResponse::Status200_AListOfSubmissions(res))
    }

    async fn update_submission(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: UpdateSubmissionPathParams,
        body: SubmissionUpdate,
    ) -> Result<UpdateSubmissionResponse, ()> {
        let res = self
            .submission_service
            .update_entity(path_params.submission_id, body)
            .await
            .expect("Failed to update submission");

        Ok(UpdateSubmissionResponse::Status200_SubmissionUpdatedSuccessfully(res))
    }

    async fn update_submission_member(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: UpdateSubmissionMemberPathParams,
        body: SubmissionMemberUpdate,
    ) -> Result<UpdateSubmissionMemberResponse, ()> {
        todo!()
    }
}
