use crate::services::core::entity_macro::generate_service;

generate_service!(
    ModuleService,
    ModuleEntity,
    modules,
    module,
    Module,
    ModuleCreate,
    ModuleUpdate,
    PageModule,
);
