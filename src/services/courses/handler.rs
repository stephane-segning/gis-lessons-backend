use crate::domain::course::CourseEntity;
use crate::modules::db::schema::courses::dsl::*;
use anyhow::Result;
use derive_builder::Builder;
use diesel::prelude::*;
use diesel::{QueryDsl, SelectableHelper};
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use gen_server::models::{CorePageMeta, Course, CourseCreate, PageCourse};

#[derive(Clone, Builder)]
pub struct CourseService {
    pool: Pool<AsyncPgConnection>,
}

impl CourseService {
    pub async fn find_course(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
        _q: Option<String>,
    ) -> Result<PageCourse> {
        let mut conn = self.pool.get().await?;
        let offset = offset.unwrap_or(0);
        let limit = limit.unwrap_or(10);

        let res = courses
            .select(CourseEntity::as_select())
            .limit(limit)
            .offset(offset)
            .load(&mut conn)
            .await?;

        let res = res.into_iter().map(|course| course.into()).collect();
        let total: i64 = courses.count().get_result(&mut conn).await?;
        
        let page_meta = CorePageMeta::new(offset, limit, total);
        Ok(PageCourse::new(page_meta, res))
    }

    pub async fn create_course(&self, course: CourseCreate) -> Result<Course> {
        let mut conn = self.pool.get().await?;
        let entity = CourseEntity::from(course);
        let res: CourseEntity = diesel::insert_into(courses)
            .values(&entity)
            .get_result(&mut conn)
            .await?;
        Ok(res.into())
    }

    pub async fn get_course(&self, course_id: String) -> Result<Course> {
        let mut conn = self.pool.get().await?;
        let res: CourseEntity = courses.filter(id.eq(course_id)).first(&mut conn).await?;
        Ok(res.into())
    }

    pub async fn update_course(
        &self,
        course_id: String,
        course: gen_server::models::CourseUpdate,
    ) -> Result<Course> {
        let mut conn = self.pool.get().await?;
        let res = CourseEntity::from(course);
        let res: CourseEntity = diesel::update(courses.filter(id.eq(course_id)))
            .set(&res)
            .get_result(&mut conn)
            .await?;

        Ok(res.into())
    }

    pub async fn delete_course(&self, _id: String) -> Result<()> {
        let mut conn = self.pool.get().await?;
        diesel::delete(courses.filter(id.eq(_id)))
            .execute(&mut conn)
            .await?;
        Ok(())
    }
}
