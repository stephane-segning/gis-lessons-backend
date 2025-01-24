use crate::services::core::entity_macro::generate_service;

generate_service!(
    EnrollmentService,
    EnrollmentEntity,
    enrollments,
    enrollment,
    Enrollment,
    EnrollmentCreate,
    EnrollmentUpdate,
    PageEnrollment,
);
