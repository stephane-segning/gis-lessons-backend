use crate::services::activities::handler::ActivityService;
use crate::services::assignments::handler::AssignmentService;
use crate::services::comments::handler::CommentService;
use crate::services::courses::handler::CourseService;
use crate::services::enrollments::handler::EnrollmentService;
use crate::services::lessons::handler::LessonService;
use crate::services::modules::handler::ModuleService;
use crate::services::submissions::handler::SubmissionService;
use derive_builder::Builder;

#[derive(Clone, Builder)]
pub struct ApiService {
    pub activity_service: ActivityService,
    pub assignment_service: AssignmentService,
    pub comment_service: CommentService,
    pub course_service: CourseService,
    pub enrollment_service: EnrollmentService,
    pub lesson_service: LessonService,
    pub module_service: ModuleService,
    pub submission_service: SubmissionService,
}

impl AsRef<ApiService> for ApiService {
    fn as_ref(&self) -> &ApiService {
        self
    }
}
