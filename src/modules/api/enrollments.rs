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
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        body: EnrollmentCreate,
    ) -> Result<CreateEnrollmentResponse, ()> {
        let res = self
            .enrollment_service
            .create_entity(body)
            .await
            .expect("Failed to create enrollment");

        Ok(CreateEnrollmentResponse::Status201_EnrollmentCreatedSuccessfully(res))
    }

    async fn delete_enrollment(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: DeleteEnrollmentPathParams,
    ) -> Result<DeleteEnrollmentResponse, ()> {
        self.enrollment_service
            .delete_entity(path_params.enrollment_id)
            .await
            .expect("Failed to delete enrollment");

        Ok(DeleteEnrollmentResponse::Status204_EnrollmentDeletedSuccessfully)
    }

    async fn get_enrollment(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: GetEnrollmentPathParams,
    ) -> Result<GetEnrollmentResponse, ()> {
        let res = self
            .enrollment_service
            .get_entity(path_params.enrollment_id)
            .await
            .expect("Failed to get enrollment");

        Ok(GetEnrollmentResponse::Status200_TheRequestedEnrollment(res))
    }

    async fn list_enrollments(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        query_params: ListEnrollmentsQueryParams,
    ) -> Result<ListEnrollmentsResponse, ()> {
        let res = self
            .enrollment_service
            .find_entity(query_params.limit, query_params.offset, query_params.q)
            .await
            .expect("Failed to find enrollment");

        Ok(ListEnrollmentsResponse::Status200_AListOfEnrollments(res))
    }

    async fn update_enrollment(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: UpdateEnrollmentPathParams,
        body: EnrollmentUpdate,
    ) -> Result<UpdateEnrollmentResponse, ()> {
        let res = self
            .enrollment_service
            .update_entity(path_params.enrollment_id, body)
            .await
            .expect("Failed to update enrollment");

        Ok(UpdateEnrollmentResponse::Status200_EnrollmentUpdatedSuccessfully(res))
    }
}
