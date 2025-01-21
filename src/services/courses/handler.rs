use crate::services::core::entity_macro::generate_service;

// Example usage of the macro:
generate_service!(
    CourseService,
    CourseEntity,
    courses,
    course,
    Course,
    CourseCreate,
    CourseUpdate,
    PageCourse,
);
