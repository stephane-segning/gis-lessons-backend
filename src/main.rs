use crate::modules::api::handler::ApiServiceBuilder;
use crate::modules::db::connection::get_connection;
use crate::modules::env::env::EnvConfig;
use crate::modules::router::router::router;
use crate::modules::tracer::init::init_tracing;
use crate::services::activities::handler::ActivityServiceBuilder;
use crate::services::courses::handler::CourseServiceBuilder;
use envconfig::Envconfig;
use opentelemetry::global;
use services::{
    assignments::handler::AssignmentServiceBuilder, comments::handler::CommentServiceBuilder,
    enrollments::handler::EnrollmentServiceBuilder, lessons::handler::LessonServiceBuilder,
    modules::handler::ModuleServiceBuilder,
    submission_members::handler::SubmissionMemberServiceBuilder,
    submissions::handler::SubmissionServiceBuilder,
};
use std::{net::SocketAddr, sync::Arc};
use tokio::{net::TcpListener, sync::RwLock};
use tracing::{debug, warn};

mod domain;
mod modules;
mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = EnvConfig::init_from_env()?;

    // Initialize tracing and OpenTelemetry
    let metrics = init_tracing(config.clone()).await?;
    let db_pool = get_connection(config.clone()).await?;
    let db_pool = Arc::new(RwLock::new(db_pool));

    // Get address to listen on
    let addr = format!("{}:{:?}", config.http_host, config.http_port).parse::<SocketAddr>()?;
    let listener = TcpListener::bind(addr).await?;
    debug!(config.http_port, config.http_host, "Will start");

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

    let app = router(metrics, api_service).await?;

    // Start the server
    warn!("Server running on http://{:?}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    // Shutdown the tracer provider
    global::shutdown_tracer_provider();
    Ok(())
}
