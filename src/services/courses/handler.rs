use crate::services::core::entity_macro::generate_service;

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
