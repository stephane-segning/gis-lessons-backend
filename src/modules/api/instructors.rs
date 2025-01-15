use crate::modules::api::handler::ApiService;
use axum::async_trait;
use axum::extract::Host;
use axum::http::Method;
use axum_extra::extract::CookieJar;
use gen_server::apis::instructors::{
    CreateInstructorResponse, DeleteInstructorResponse, GetInstructorResponse, Instructors,
    ListInstructorsResponse, UpdateInstructorResponse,
};
use gen_server::models::{
    DeleteInstructorPathParams, GetInstructorPathParams,
    InstructorCreate, InstructorUpdate, ListInstructorsQueryParams, UpdateInstructorPathParams,
};

#[async_trait]
impl Instructors for ApiService {
    async fn create_instructor(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _body: InstructorCreate,
    ) -> Result<CreateInstructorResponse, ()> {
        todo!()
    }

    async fn delete_instructor(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _path_params: DeleteInstructorPathParams,
    ) -> Result<DeleteInstructorResponse, ()> {
        todo!()
    }

    async fn get_instructor(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _path_params: GetInstructorPathParams,
    ) -> Result<GetInstructorResponse, ()> {
        todo!()
    }

    async fn list_instructors(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _query_params: ListInstructorsQueryParams,
    ) -> Result<ListInstructorsResponse, ()> {
        todo!()
    }

    async fn update_instructor(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _path_params: UpdateInstructorPathParams,
        _body: InstructorUpdate,
    ) -> Result<UpdateInstructorResponse, ()> {
        todo!()
    }
}
