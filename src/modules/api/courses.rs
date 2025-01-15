use crate::modules::api::handler::ApiService;
use axum::async_trait;
use axum::extract::Host;
use axum::http::Method;
use axum_extra::extract::CookieJar;
use gen_server::apis::courses::{
    Courses, CreateCourseResponse, DeleteCourseResponse, GetCourseResponse, ListCoursesResponse,
    UpdateCourseResponse,
};
use gen_server::models::{
    CourseCreate, CourseUpdate, DeleteCoursePathParams, GetCoursePathParams,
    ListCoursesQueryParams, UpdateCoursePathParams,
};

#[async_trait]
impl Courses for ApiService {
    async fn create_course(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _body: CourseCreate,
    ) -> Result<CreateCourseResponse, ()> {
        todo!()
    }

    async fn delete_course(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _path_params: DeleteCoursePathParams,
    ) -> Result<DeleteCourseResponse, ()> {
        todo!()
    }

    async fn get_course(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _path_params: GetCoursePathParams,
    ) -> Result<GetCourseResponse, ()> {
        todo!()
    }

    async fn list_courses(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _query_params: ListCoursesQueryParams,
    ) -> Result<ListCoursesResponse, ()> {
        todo!()
    }

    async fn update_course(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _path_params: UpdateCoursePathParams,
        _body: CourseUpdate,
    ) -> Result<UpdateCourseResponse, ()> {
        todo!()
    }
}
