use crate::modules::api::handler::ApiService;
use axum::async_trait;
use axum::extract::Host;
use axum::http::Method;
use axum_extra::extract::CookieJar;
use gen_server::apis::lessons::{
    CreateLessonResponse, DeleteLessonResponse, GetLessonBlocksResponse, GetLessonResponse,
    Lessons, ListLessonResponse, UpdateLessonResponse,
};
use gen_server::models::{
    DeleteLessonPathParams, GetLessonBlocksPathParams, GetLessonPathParams
    , LessonCreate, LessonUpdate, ListLessonPathParams,
    ListLessonQueryParams, UpdateLessonPathParams,
};

#[async_trait]
impl Lessons for ApiService {
    async fn create_lesson(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        body: LessonCreate,
    ) -> Result<CreateLessonResponse, ()> {
        todo!()
    }

    async fn delete_lesson(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: DeleteLessonPathParams,
    ) -> Result<DeleteLessonResponse, ()> {
        todo!()
    }

    async fn get_lesson(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: GetLessonPathParams,
    ) -> Result<GetLessonResponse, ()> {
        todo!()
    }

    async fn get_lesson_blocks(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: GetLessonBlocksPathParams,
    ) -> Result<GetLessonBlocksResponse, ()> {
        todo!()
    }

    async fn list_lesson(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: ListLessonPathParams,
        query_params: ListLessonQueryParams,
    ) -> Result<ListLessonResponse, ()> {
        todo!()
    }

    async fn update_lesson(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: UpdateLessonPathParams,
        body: LessonUpdate,
    ) -> Result<UpdateLessonResponse, ()> {
        todo!()
    }
}
