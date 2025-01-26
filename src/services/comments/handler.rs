use crate::services::core::entity_macro::generate_service;

generate_service!(
    CommentService,
    CommentEntity,
    comments,
    comment,
    Comment,
    CommentCreate,
    CommentUpdate,
    PageComment,
);
