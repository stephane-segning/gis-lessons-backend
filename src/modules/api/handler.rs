use crate::services::activities::handler::{ActivityService, ActivityServiceBuilder};
use crate::services::assignments::handler::{AssignmentService, AssignmentServiceBuilder};
use crate::services::comments::handler::{CommentService, CommentServiceBuilder};
use crate::services::courses::handler::{CourseService, CourseServiceBuilder};
use crate::services::enrollments::handler::{EnrollmentService, EnrollmentServiceBuilder};
use crate::services::lessons::handler::{LessonService, LessonServiceBuilder};
use crate::services::modules::handler::{ModuleService, ModuleServiceBuilder};
use crate::services::submission_members::handler::{
    SubmissionMemberService, SubmissionMemberServiceBuilder,
};
use crate::services::submissions::handler::{SubmissionService, SubmissionServiceBuilder};
use anyhow::Result;
use derive_builder::Builder;
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::AsyncPgConnection;
use std::sync::Arc;
use tokio::sync::RwLock;

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
    pub submission_member_service: SubmissionMemberService,
}

impl ApiService {
    pub fn create(db_pool: Arc<RwLock<Pool<AsyncPgConnection>>>) -> Result<Self> {
        let activity_service = ActivityServiceBuilder::default()
            .pool(db_pool.clone())
            .build()?;
        let assignment_service = AssignmentServiceBuilder::default()
            .pool(db_pool.clone())
            .build()?;
        let comment_service = CommentServiceBuilder::default()
            .pool(db_pool.clone())
            .build()?;
        let course_service = CourseServiceBuilder::default()
            .pool(db_pool.clone())
            .build()?;
        let enrollment_service = EnrollmentServiceBuilder::default()
            .pool(db_pool.clone())
            .build()?;
        let lesson_service = LessonServiceBuilder::default()
            .pool(db_pool.clone())
            .build()?;
        let module_service = ModuleServiceBuilder::default()
            .pool(db_pool.clone())
            .build()?;
        let submission_service = SubmissionServiceBuilder::default()
            .pool(db_pool.clone())
            .build()?;
        let submission_member_service = SubmissionMemberServiceBuilder::default()
            .pool(db_pool)
            .build()?;

        let api_service = ApiServiceBuilder::default()
            .activity_service(activity_service)
            .assignment_service(assignment_service)
            .comment_service(comment_service)
            .course_service(course_service)
            .enrollment_service(enrollment_service)
            .lesson_service(lesson_service)
            .module_service(module_service)
            .submission_service(submission_service)
            .submission_member_service(submission_member_service)
            .build()?;

        Ok(api_service)
    }
}

impl AsRef<ApiService> for ApiService {
    fn as_ref(&self) -> &ApiService {
        self
    }
}
