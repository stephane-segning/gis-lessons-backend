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
    ListSubmissionMembersQueryParams, ListSubmissionsQueryParams, SubmissionCreate,
    SubmissionMemberCreate, SubmissionMemberUpdate, SubmissionUpdate,
    UpdateSubmissionMemberPathParams, UpdateSubmissionPathParams,
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
        _path_params: CreateSubmissionMemberPathParams,
        body: SubmissionMemberCreate,
    ) -> Result<CreateSubmissionMemberResponse, ()> {
        let res = self
            .submission_member_service
            .create_entity(body)
            .await
            .expect("Failed to create submission member");

        Ok(CreateSubmissionMemberResponse::Status201_SubmissionCreatedSuccessfully(res))
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
        self.submission_member_service
            .delete_entity(path_params.submission_id, path_params.enrollment_id)
            .await
            .expect("Failed to delete submission member");

        Ok(DeleteSubmissionMemberResponse::Status204_Submission)
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
        let res = self
            .submission_member_service
            .get_entity(path_params.submission_id, path_params.enrollment_id)
            .await
            .expect("Failed to get submission member");

        Ok(GetSubmissionMemberResponse::Status200_TheRequestedMember(
            res,
        ))
    }

    async fn list_submission_members(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: ListSubmissionMembersPathParams,
        query_params: ListSubmissionMembersQueryParams,
    ) -> Result<ListSubmissionMembersResponse, ()> {
        let res = self
            .submission_member_service
            .find_entity(
                path_params.submission_id,
                query_params.limit,
                query_params.offset,
                query_params.q,
            )
            .await
            .expect("Failed to find submission member");

        Ok(ListSubmissionMembersResponse::Status200_AListOfMembers(res))
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
        let res = self
            .submission_member_service
            .update_entity(path_params.submission_id, path_params.enrollment_id, body)
            .await
            .expect("Failed to update submission member");

        Ok(UpdateSubmissionMemberResponse::Status200_SubmissionUpdatedSuccessfully(res))
    }
}
