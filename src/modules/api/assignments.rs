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
        _body: AssignmentCreate,
    ) -> Result<CreateAssignmentResponse, ()> {
        todo!()
    }

    async fn delete_assignment(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _path_params: DeleteAssignmentPathParams,
    ) -> Result<DeleteAssignmentResponse, ()> {
        todo!()
    }

    async fn get_assignment(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _path_params: GetAssignmentPathParams,
    ) -> Result<GetAssignmentResponse, ()> {
        todo!()
    }

    async fn list_assignments(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _query_params: ListAssignmentsQueryParams,
    ) -> Result<ListAssignmentsResponse, ()> {
        todo!()
    }

    async fn update_assignment(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _path_params: UpdateAssignmentPathParams,
        _body: AssignmentUpdate,
    ) -> Result<UpdateAssignmentResponse, ()> {
        todo!()
    }
}
