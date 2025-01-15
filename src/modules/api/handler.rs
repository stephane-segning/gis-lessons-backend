use crate::services::activities::handler::ActivityService;
use crate::services::courses::handler::CourseService;
use derive_builder::Builder;

#[derive(Clone, Builder)]
pub struct ApiService {
    pub activity_service: ActivityService,
    pub course_service: CourseService,
}

impl AsRef<ApiService> for ApiService {
    fn as_ref(&self) -> &ApiService {
        self
    }
}
