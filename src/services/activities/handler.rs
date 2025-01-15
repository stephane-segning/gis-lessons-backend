use crate::domain::activity::ActivityEntity;
use crate::modules::db::schema::activities::dsl::activities;
use anyhow::Result;
use derive_builder::Builder;
use diesel::prelude::*;
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use gen_server::models::{CorePageMeta, PageActivity};

#[derive(Clone, Builder)]
pub struct ActivityService {
    pool: Pool<AsyncPgConnection>,
}

impl ActivityService {
    pub async fn find_activities(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
        _q: Option<String>,
    ) -> Result<PageActivity> {
        let mut conn = self.pool.get().await?;
        let offset = offset.unwrap_or(0);
        let limit = limit.unwrap_or(10);

        let res = activities
            .select(ActivityEntity::as_select())
            .limit(limit)
            .offset(offset)
            .load(&mut conn)
            .await?;

        let res = res.into_iter().map(|activity| activity.into()).collect();
        let total: i64 = activities.count().get_result(&mut conn).await?;

        let page_meta = CorePageMeta::new(offset, limit, total);
        Ok(PageActivity::new(page_meta, res))
    }
}
