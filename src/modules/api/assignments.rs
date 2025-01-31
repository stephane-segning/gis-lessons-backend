use crate::modules::api::handler::ApiService;
use axum::async_trait;
use axum::extract::Host;
use axum::http::Method;
use axum_extra::extract::CookieJar;
use gen_server::apis::assignments::{
    Assignments, CreateAssignmentResponse, DeleteAssignmentResponse, GetAssignmentResponse,
    ListAssignmentsResponse, UpdateAssignmentResponse,
};
use gen_server::models::{
    AssignmentCreate, AssignmentUpdate, DeleteAssignmentPathParams, GetAssignmentPathParams,
    ListAssignmentsQueryParams, UpdateAssignmentPathParams,
};

#[async_trait]
impl Assignments for ApiService {
    async fn create_assignment(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        body: AssignmentCreate,
    ) -> Result<CreateAssignmentResponse, ()> {
        let res = self
            .assignment_service
            .create_entity(body)
            .await
            .expect("Failed to create assignment");

        Ok(CreateAssignmentResponse::Status201_AssignmentCreatedSuccessfully(res))
    }

    async fn delete_assignment(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: DeleteAssignmentPathParams,
    ) -> Result<DeleteAssignmentResponse, ()> {
        self.assignment_service
            .delete_entity(path_params.assignment_id)
            .await
            .expect("Failed to delete assignment");

        Ok(DeleteAssignmentResponse::Status204_AssignmentDeletedSuccessfully)
    }

    async fn get_assignment(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: GetAssignmentPathParams,
    ) -> Result<GetAssignmentResponse, ()> {
        let res = self
            .assignment_service
            .get_entity(path_params.assignment_id)
            .await
            .expect("Failed to get assignment");

        Ok(GetAssignmentResponse::Status200_TheRequestedAssignment(res))
    }

    async fn list_assignments(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        query_params: ListAssignmentsQueryParams,
    ) -> Result<ListAssignmentsResponse, ()> {
        let res = self
            .assignment_service
            .find_entity(query_params.limit, query_params.offset, query_params.q)
            .await
            .expect("Failed to find assignment");

        Ok(ListAssignmentsResponse::Status200_AListOfAssignments(res))
    }

    async fn update_assignment(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: UpdateAssignmentPathParams,
        body: AssignmentUpdate,
    ) -> Result<UpdateAssignmentResponse, ()> {
        let res = self
            .assignment_service
            .update_entity(path_params.assignment_id, body)
            .await
            .expect("Failed to update assignment");

        Ok(UpdateAssignmentResponse::Status200_AssignmentUpdatedSuccessfully(res))
    }
}
