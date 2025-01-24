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
    DeleteModulePathParams, GetModulePathParams, ListModulesQueryParams, ModuleCreate,
    ModuleUpdate, UpdateModulePathParams,
};

#[async_trait]
impl Modules for ApiService {
    async fn create_module(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        body: ModuleCreate,
    ) -> Result<CreateModuleResponse, ()> {
        let res = self
            .module_service
            .create_entity(body)
            .await
            .expect("Failed to create module");

        Ok(CreateModuleResponse::Status201_ModuleCreatedSuccessfully(
            res,
        ))
    }

    async fn delete_module(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: DeleteModulePathParams,
    ) -> Result<DeleteModuleResponse, ()> {
        self.module_service
            .delete_entity(path_params.module_id)
            .await
            .expect("Failed to delete module");

        Ok(DeleteModuleResponse::Status204_ModuleDeletedSuccessfully)
    }

    async fn get_module(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: GetModulePathParams,
    ) -> Result<GetModuleResponse, ()> {
        let res = self
            .module_service
            .get_entity(path_params.module_id)
            .await
            .expect("Failed to get module");

        Ok(GetModuleResponse::Status200_TheRequestedModule(res))
    }

    async fn list_modules(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        query_params: ListModulesQueryParams,
    ) -> Result<ListModulesResponse, ()> {
        let res = self
            .module_service
            .find_entity(query_params.limit, query_params.offset, query_params.q)
            .await
            .expect("Failed to find module");

        Ok(ListModulesResponse::Status200_AListOfModules(res))
    }

    async fn update_module(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: UpdateModulePathParams,
        body: ModuleUpdate,
    ) -> Result<UpdateModuleResponse, ()> {
        let res = self
            .module_service
            .update_entity(path_params.module_id, body)
            .await
            .expect("Failed to update module");

        Ok(UpdateModuleResponse::Status200_ModuleUpdatedSuccessfully(
            res,
        ))
    }
}
