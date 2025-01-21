macro_rules! generate_service {
    (
        $service_name:ident,
        $entity_name:ident,
        $table_name:ident,
        $dsl_mod:ident,
        $model:ident,
        $create_model:ident,
        $update_model:ident,
        $page_meta:ident,
    ) => {
        use crate::domain::$dsl_mod::$entity_name;
        use crate::modules::db::schema::$table_name::dsl::*;
        use anyhow::Result;
        use derive_builder::Builder;
        use diesel::prelude::*;
        use diesel::{QueryDsl, SelectableHelper};
        use diesel_async::pooled_connection::deadpool::Pool;
        use diesel_async::{AsyncPgConnection, RunQueryDsl};
        use gen_server::models::{
            CorePageMeta, $model, $update_model, $create_model, $page_meta
        };

        #[derive(Clone, Builder)]
        pub struct $service_name {
            pool: Pool<AsyncPgConnection>,
        }

        impl $service_name {
            pub async fn find_entity(
                &self,
                limit: Option<i64>,
                offset: Option<i64>,
                _q: Option<String>,
            ) -> Result<$page_meta> {
                let mut conn = self.pool.get().await?;
                let offset = offset.unwrap_or(0);
                let limit = limit.unwrap_or(10);
                                
                let res = $table_name
                    .select($entity_name::as_select())
                    .limit(limit)
                    .offset(offset)
                    .load(&mut conn)
                    .await?;

                let res = res.into_iter().map(|entity| entity.into()).collect();
                let total: i64 = $table_name.count().get_result(&mut conn).await?;
                
                let page_meta = CorePageMeta::new(offset, limit, total);
                Ok($page_meta::new(page_meta, res))
            }

            pub async fn create_entity(&self, entity: $create_model) -> Result<$model> {
                let mut conn = self.pool.get().await?;
                let entity = $entity_name::from(entity);
                let res: $entity_name = diesel::insert_into($table_name)
                    .values(&entity)
                    .get_result(&mut conn)
                    .await?;
                Ok(res.into())
            }

            pub async fn get_entity(&self, entity_id: String) -> Result<$model> {
                let mut conn = self.pool.get().await?;
                let res: $entity_name = $table_name.filter(id.eq(entity_id)).first(&mut conn).await?;
                Ok(res.into())
            }

            pub async fn update_entity(
                &self,
                entity_id: String,
                entity: $update_model,
            ) -> Result<$model> {
                let mut conn = self.pool.get().await?;
                let res = $entity_name::from(entity);
                let res: $entity_name = diesel::update($table_name.filter(id.eq(entity_id)))
                    .set(&res)
                    .get_result(&mut conn)
                    .await?;

                Ok(res.into())
            }

            pub async fn delete_entity(&self, entity_id: String) -> Result<()> {
                let mut conn = self.pool.get().await?;
                diesel::delete($table_name.filter(id.eq(entity_id)))
                    .execute(&mut conn)
                    .await?;
                Ok(())
            }
        }
    };
}

pub(crate) use generate_service;