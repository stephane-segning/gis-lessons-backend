use crate::modules::api::handler::ApiService;
use axum::async_trait;
use axum::extract::Host;
use axum::http::Method;
use axum_extra::extract::CookieJar;
use gen_server::apis::students::{
    CreateStudentResponse, DeleteStudentResponse, GetStudentResponse, ListStudentsResponse,
    Students, UpdateStudentResponse,
};
use gen_server::models::{
    DeleteStudentPathParams, GetStudentPathParams, ListStudentsQueryParams, StudentCreate,
    StudentUpdate, UpdateStudentPathParams,
};

#[async_trait]
impl Students for ApiService {
    async fn create_student(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _body: StudentCreate,
    ) -> Result<CreateStudentResponse, ()> {
        todo!()
    }

    async fn delete_student(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _path_params: DeleteStudentPathParams,
    ) -> Result<DeleteStudentResponse, ()> {
        todo!()
    }

    async fn get_student(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _path_params: GetStudentPathParams,
    ) -> Result<GetStudentResponse, ()> {
        todo!()
    }

    async fn list_students(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _query_params: ListStudentsQueryParams,
    ) -> Result<ListStudentsResponse, ()> {
        todo!()
    }

    async fn update_student(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _path_params: UpdateStudentPathParams,
        _body: StudentUpdate,
    ) -> Result<UpdateStudentResponse, ()> {
        todo!()
    }
}
