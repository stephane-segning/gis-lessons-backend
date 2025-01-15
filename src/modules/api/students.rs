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
    DeleteStudentPathParams, GetStudentPathParams,
    ListStudentsQueryParams, StudentCreate, StudentUpdate, UpdateStudentPathParams,
};

#[async_trait]
impl Students for ApiService {
    async fn create_student(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        body: StudentCreate,
    ) -> Result<CreateStudentResponse, ()> {
        todo!()
    }

    async fn delete_student(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: DeleteStudentPathParams,
    ) -> Result<DeleteStudentResponse, ()> {
        todo!()
    }

    async fn get_student(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: GetStudentPathParams,
    ) -> Result<GetStudentResponse, ()> {
        todo!()
    }

    async fn list_students(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        query_params: ListStudentsQueryParams,
    ) -> Result<ListStudentsResponse, ()> {
        todo!()
    }

    async fn update_student(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: UpdateStudentPathParams,
        body: StudentUpdate,
    ) -> Result<UpdateStudentResponse, ()> {
        todo!()
    }
}
