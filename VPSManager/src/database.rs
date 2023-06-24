use rocket::request::Outcome;
use rocket::request::FromRequest;
use rocket::State;
use sqlx::{PgPool, Pool, Postgres, Connection};

pub type DbConn = PgPool;

pub async fn connect() -> DbConn {
    let pool = PgPool::connect("postgres://username:password@localhost/database_name")
        .await
        .expect("Failed to connect to the database.");
    pool
}

pub struct DbConnGuard(pub DbConn);

#[rocket::async_trait]
impl<'a, 'r> FromRequest<'a, 'r> for DbConnGuard {
    type Error = ();

    async fn from_request(request: &'a rocket::Request<'r>) -> rocket::request::Outcome<Self, Self::Error> {
        let pool = request.guard::<State<DbConn>>().await.expect("Database pool not found.");
        Outcome::Success(DbConnGuard(pool.clone()))
    }
}
