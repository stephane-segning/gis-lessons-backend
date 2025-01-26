use crate::services::core::entity_macro::generate_service;

generate_service!(
    SubmissionService,
    SubmissionEntity,
    submissions,
    submission,
    Submission,
    SubmissionCreate,
    SubmissionUpdate,
    PageSubmission,
);
