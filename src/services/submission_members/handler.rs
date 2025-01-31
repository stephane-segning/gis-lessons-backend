use std::sync::Arc;

use crate::domain::submission_member::SubmissionMemberEntity;
use crate::modules::db::schema::submission_members::dsl::*;
use anyhow::Result;
use derive_builder::Builder;
use diesel::prelude::*;
use diesel::{QueryDsl, SelectableHelper};
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use gen_server::models::{
    CorePageMeta, PageSubmissionMember, SubmissionMember, SubmissionMemberCreate,
    SubmissionMemberUpdate,
};
use tokio::sync::RwLock;

#[derive(Clone, Builder)]
pub struct SubmissionMemberService {
    pool: Arc<RwLock<Pool<AsyncPgConnection>>>,
}

impl SubmissionMemberService {
    pub async fn find_entity(
        &self,
        entity_submission_id: String,
        limit: Option<i64>,
        offset: Option<i64>,
        _q: Option<String>,
    ) -> Result<PageSubmissionMember> {
        let pool = self.pool.read().await;
        let mut conn = pool.get().await?;
        let offset = offset.unwrap_or(0);
        let limit = limit.unwrap_or(10);

        let res = submission_members
            .select(SubmissionMemberEntity::as_select())
            .filter(submission_id.eq(entity_submission_id))
            .limit(limit)
            .offset(offset)
            .load(&mut conn)
            .await?;

        let res = res.into_iter().map(|entity| entity.into()).collect();
        let total: i64 = submission_members.count().get_result(&mut conn).await?;

        let page_meta = CorePageMeta::new(limit, offset, total);
        Ok(PageSubmissionMember::new(page_meta, res))
    }

    pub async fn create_entity(&self, entity: SubmissionMemberCreate) -> Result<SubmissionMember> {
        let pool = self.pool.read().await;
        let mut conn = pool.get().await?;

        let entity = SubmissionMemberEntity::from(entity);
        let res: SubmissionMemberEntity = diesel::insert_into(submission_members)
            .values(&entity)
            .get_result(&mut conn)
            .await?;
        Ok(res.into())
    }

    pub async fn get_entity(
        &self,
        entity_submission_id: String,
        entity_enrollment_id: String,
    ) -> Result<SubmissionMember> {
        let pool = self.pool.read().await;
        let mut conn = pool.get().await?;

        let res: SubmissionMemberEntity = submission_members
            .filter(submission_id.eq(entity_submission_id))
            .filter(enrollment_id.eq(entity_enrollment_id))
            .first(&mut conn)
            .await?;
        Ok(res.into())
    }

    pub async fn update_entity(
        &self,
        entity_submission_id: String,
        entity_enrollment_id: String,
        entity: SubmissionMemberUpdate,
    ) -> Result<SubmissionMember> {
        let pool = self.pool.read().await;
        let mut conn = pool.get().await?;

        let res = SubmissionMemberEntity::from(entity);
        let res: SubmissionMemberEntity = diesel::update(
            submission_members
                .filter(submission_id.eq(entity_submission_id))
                .filter(enrollment_id.eq(entity_enrollment_id)),
        )
        .set(&res)
        .get_result(&mut conn)
        .await?;

        Ok(res.into())
    }

    pub async fn delete_entity(
        &self,
        entity_submission_id: String,
        entity_enrollment_id: String,
    ) -> Result<()> {
        let pool = self.pool.read().await;
        let mut conn = pool.get().await?;

        diesel::delete(
            submission_members
                .filter(submission_id.eq(entity_submission_id))
                .filter(enrollment_id.eq(entity_enrollment_id)),
        )
        .execute(&mut conn)
        .await?;
        Ok(())
    }
}
