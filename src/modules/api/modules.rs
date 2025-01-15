use crate::modules::api::handler::ApiService;
use axum::async_trait;
use axum::extract::Host;
use axum::http::Method;
use axum_extra::extract::CookieJar;
use gen_server::apis::modules::{
    CreateModuleResponse, DeleteModuleResponse, GetModuleResponse, ListModulesResponse, Modules,
    UpdateModuleResponse,
};
use gen_server::models::{
    DeleteModulePathParams, GetModulePathParams,
    ListModulesQueryParams, ModuleCreate, ModuleUpdate, UpdateModulePathParams,
};

#[async_trait]
impl Modules for ApiService {
    async fn create_module(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        body: ModuleCreate,
    ) -> Result<CreateModuleResponse, ()> {
        todo!()
    }

    async fn delete_module(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: DeleteModulePathParams,
    ) -> Result<DeleteModuleResponse, ()> {
        todo!()
    }

    async fn get_module(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: GetModulePathParams,
    ) -> Result<GetModuleResponse, ()> {
        todo!()
    }

    async fn list_modules(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        query_params: ListModulesQueryParams,
    ) -> Result<ListModulesResponse, ()> {
        todo!()
    }

    async fn update_module(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: UpdateModulePathParams,
        body: ModuleUpdate,
    ) -> Result<UpdateModuleResponse, ()> {
        todo!()
    }
}
