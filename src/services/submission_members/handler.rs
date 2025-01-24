use crate::domain::submission_member::SubmissionMemberEntity;
use crate::modules::db::schema::submission_members::dsl::submission_members;
use anyhow::Result;
use derive_builder::Builder;
use diesel::prelude::*;
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use gen_server::models::SubmissionMember;

#[derive(Clone, Builder)]
pub struct SubmissionMemberService {
    pool: Pool<AsyncPgConnection>,
}

impl SubmissionMemberService {
    pub async fn find_submission_members(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
        _q: Option<String>,
    ) -> Result<Vec<SubmissionMember>> {
        let mut conn = self.pool.get().await?;
        let offset = offset.unwrap_or(0);
        let limit = limit.unwrap_or(10);

        let res = submission_members
            .select(SubmissionMemberEntity::as_select())
            .limit(limit)
            .offset(offset)
            .load(&mut conn)
            .await?;

        let res = res
            .into_iter()
            .map(|submission_member| submission_member.into())
            .collect();
        Ok(res)
    }
}
