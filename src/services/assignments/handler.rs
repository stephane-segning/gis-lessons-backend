use crate::services::core::entity_macro::generate_service;

generate_service!(
    AssignmentService,
    AssignmentEntity,
    assignments,
    assignment,
    Assignment,
    AssignmentCreate,
    AssignmentUpdate,
    PageAssignment,
);
