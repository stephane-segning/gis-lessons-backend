use crate::modules::api::handler::ApiService;
use axum::async_trait;
use axum::extract::Host;
use axum::http::Method;
use axum_extra::extract::CookieJar;
use gen_server::apis::enrollments::{
    CreateEnrollmentResponse, DeleteEnrollmentResponse, Enrollments, GetEnrollmentResponse,
    ListEnrollmentsResponse, UpdateEnrollmentResponse,
};
use gen_server::models::{
    DeleteEnrollmentPathParams, EnrollmentCreate, EnrollmentUpdate, GetEnrollmentPathParams,
    ListEnrollmentsQueryParams, UpdateEnrollmentPathParams,
};

#[async_trait]
impl Enrollments for ApiService {
    async fn create_enrollment(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        body: EnrollmentCreate,
    ) -> Result<CreateEnrollmentResponse, ()> {
        todo!()
    }

    async fn delete_enrollment(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: DeleteEnrollmentPathParams,
    ) -> Result<DeleteEnrollmentResponse, ()> {
        todo!()
    }

    async fn get_enrollment(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: GetEnrollmentPathParams,
    ) -> Result<GetEnrollmentResponse, ()> {
        todo!()
    }

    async fn list_enrollments(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        query_params: ListEnrollmentsQueryParams,
    ) -> Result<ListEnrollmentsResponse, ()> {
        todo!()
    }

    async fn update_enrollment(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: UpdateEnrollmentPathParams,
        body: EnrollmentUpdate,
    ) -> Result<UpdateEnrollmentResponse, ()> {
        todo!()
    }
}
