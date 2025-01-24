use crate::modules::api::handler::ApiService;
use axum::async_trait;
use axum::extract::Host;
use axum::http::Method;
use axum_extra::extract::CookieJar;
use gen_server::apis::lessons::{
    CreateLessonResponse, DeleteLessonResponse, GetLessonResponse, Lessons, ListLessonResponse,
    UpdateLessonResponse,
};
use gen_server::models::{
    DeleteLessonPathParams, GetLessonPathParams, LessonCreate, LessonUpdate, ListLessonQueryParams,
    UpdateLessonPathParams,
};

#[async_trait]
impl Lessons for ApiService {
    async fn create_lesson(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        body: LessonCreate,
    ) -> Result<CreateLessonResponse, ()> {
        let res = self
            .lesson_service
            .create_entity(body)
            .await
            .expect("Failed to create lesson");

        Ok(CreateLessonResponse::Status201_LessonCreatedSuccessfully(
            res,
        ))
    }

    async fn delete_lesson(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: DeleteLessonPathParams,
    ) -> Result<DeleteLessonResponse, ()> {
        self.lesson_service
            .delete_entity(path_params.lesson_id)
            .await
            .expect("Failed to delete lesson");

        Ok(DeleteLessonResponse::Status204_LessonDeletedSuccessfully)
    }

    async fn get_lesson(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: GetLessonPathParams,
    ) -> Result<GetLessonResponse, ()> {
        let res = self
            .lesson_service
            .get_entity(path_params.lesson_id)
            .await
            .expect("Failed to get lesson");

        Ok(GetLessonResponse::Status200_TheRequestedLesson(res))
    }

    async fn list_lesson(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        query_params: ListLessonQueryParams,
    ) -> Result<ListLessonResponse, ()> {
        let res = self
            .lesson_service
            .find_entity(query_params.limit, query_params.offset, query_params.q)
            .await
            .expect("Failed to find lesson");

        Ok(ListLessonResponse::Status200_AListOfLessons(res))
    }

    async fn update_lesson(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: UpdateLessonPathParams,
        body: LessonUpdate,
    ) -> Result<UpdateLessonResponse, ()> {
        let res = self
            .lesson_service
            .update_entity(path_params.lesson_id, body)
            .await
            .expect("Failed to update lesson");

        Ok(UpdateLessonResponse::Status200_LessonUpdatedSuccessfully(
            res,
        ))
    }
}
