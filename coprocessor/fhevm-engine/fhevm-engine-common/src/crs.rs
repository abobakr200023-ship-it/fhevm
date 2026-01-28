use anyhow::Result;
use sqlx::{PgPool, Row};
use std::{collections::HashMap, sync::Arc};

use crate::utils::safe_deserialize_key;

pub type CrsId = Vec<u8>;

#[derive(Clone)]
pub struct Crs {
    pub sequence_number: i64,
    pub crs_id: CrsId,
    pub crs: tfhe::zk::CompactPkeCrs,
}

#[derive(Clone)]
pub struct CrsCache {
    by_id: HashMap<CrsId, Crs>,
    latest: Option<Crs>,
}

impl CrsCache {
    pub async fn load(pool: &PgPool) -> Result<Arc<Self>> {
        let rows = sqlx::query("SELECT sequence_number, crs_id, crs FROM crs")
            .fetch_all(pool)
            .await?;

        let mut by_id = HashMap::with_capacity(rows.len());
        let mut latest: Option<Crs> = None;

        for row in rows {
            let crs = Crs {
                sequence_number: row.try_get("sequence_number")?,
                crs_id: row.try_get("crs_id")?,
                crs: safe_deserialize_key(row.try_get("crs")?)?,
            };

            if latest.is_none() || crs.sequence_number > latest.as_ref().unwrap().sequence_number {
                latest = Some(crs.clone());
            }

            by_id.insert(crs.crs_id.clone(), crs);
        }

        Ok(Arc::new(Self { by_id, latest }))
    }

    pub fn get_by_id(&self, crs_id: &[u8]) -> Option<&Crs> {
        self.by_id.get(crs_id)
    }

    pub fn get_latest(&self) -> Option<&Crs> {
        self.latest.as_ref()
    }
}
