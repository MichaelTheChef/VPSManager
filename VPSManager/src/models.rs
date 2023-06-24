use serde::{Deserialize, Serialize};
use crate::database::DbConn;
use sqlx::{Error, FromRow};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Vps {
    pub id: u32,
    pub hostname: String,
    pub ip_address: String,
}

impl Vps {
    pub async fn create(conn: &DbConn, vps: Vps) -> Result<Self, Error> {
        let inserted_vps = sqlx::query_as::<_, Vps>("INSERT INTO vps (hostname, ip_address) VALUES ($1, $2) RETURNING *")
            .bind(vps.hostname)
            .bind(vps.ip_address)
            .fetch_one(&**conn)
            .await?;
        Ok(inserted_vps)
    }

    pub async fn find(conn: &DbConn, id: u32) -> Option<Self> {
        sqlx::query_as::<_, Vps>("SELECT * FROM vps WHERE id = $1")
            .bind(id)
            .fetch_optional(&**conn)
            .await
            .ok()?
    }

    pub async fn update(conn: &DbConn, id: u32, vps: Vps) -> Result<Self, Error> {
        let updated_vps = sqlx::query_as::<_, Vps>("UPDATE vps SET hostname = $1, ip_address = $2 WHERE id = $3 RETURNING *")
            .bind(vps.hostname)
            .bind(vps.ip_address)
            .bind(id)
            .fetch_one(&**conn)
            .await?;
        Ok(updated_vps)
    }

    pub async fn delete(conn: &DbConn, id: u32) -> bool {
        let result = sqlx::query("DELETE FROM vps WHERE id = $1")
            .bind(id)
            .execute(&**conn)
            .await;
        result.is_ok() && result.unwrap().rows_affected() > 0
    }
}
