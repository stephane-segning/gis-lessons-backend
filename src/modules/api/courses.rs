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
        body: CourseCreate,
    ) -> Result<CreateCourseResponse, ()> {
        let res = self
            .course_service
            .create_entity(body)
            .await
            .expect("Failed to create course");

        Ok(CreateCourseResponse::Status201_CourseCreatedSuccessfully(
            res,
        ))
    }

    async fn delete_course(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: DeleteCoursePathParams,
    ) -> Result<DeleteCourseResponse, ()> {
        self.course_service
            .delete_entity(path_params.course_id)
            .await
            .expect("Failed to delete course");

        Ok(DeleteCourseResponse::Status204_CourseDeletedSuccessfully)
    }

    async fn get_course(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: GetCoursePathParams,
    ) -> Result<GetCourseResponse, ()> {
        let res = self
            .course_service
            .get_entity(path_params.course_id)
            .await
            .expect("Failed to get course");

        Ok(GetCourseResponse::Status200_TheRequestedCourse(res))
    }

    async fn list_courses(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        query_params: ListCoursesQueryParams,
    ) -> Result<ListCoursesResponse, ()> {
        let res = self
            .course_service
            .find_entity(query_params.limit, query_params.offset, query_params.q)
            .await
            .expect("Failed to find course");

        Ok(ListCoursesResponse::Status200_AListOfCourses(res))
    }

    async fn update_course(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: UpdateCoursePathParams,
        body: CourseUpdate,
    ) -> Result<UpdateCourseResponse, ()> {
        let res = self
            .course_service
            .update_entity(path_params.course_id, body)
            .await
            .expect("Failed to update course");

        Ok(UpdateCourseResponse::Status200_CourseUpdatedSuccessfully(
            res,
        ))
    }
}
