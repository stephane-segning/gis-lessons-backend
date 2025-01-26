use crate::services::core::entity_macro::generate_service;

generate_service!(
    LessonService,
    LessonEntity,
    lessons,
    lesson,
    Lesson,
    LessonCreate,
    LessonUpdate,
    PageLesson,
);
