use serde::{Deserialize, Serialize};
use sqlx::Row;
use sqlx::{types::Uuid, Pool, Postgres};
use std::string::String;

#[derive(Debug, Serialize, Deserialize)]
pub struct User<T> {
    pub id: T,
    pub username: String,
}
impl<T> User<T> {
    #[inline]
    fn _new(empty: T) -> Self {
        Self {
            id: empty,
            username: String::new(),
        }
    }

    #[inline]
    fn map<U>(self, f: impl FnOnce(T) -> U) -> User<U> {
        User {
            id: f(self.id),
            username: self.username,
        }
    }
}
impl User<String> {
    pub fn new() -> Self {
        Self::_new(String::new())
    }
    pub fn uuid(self) -> User<Uuid> {
        self.map(|x| Uuid::parse_str(&x).unwrap())
    }
    pub fn from_row(row: &sqlx::postgres::PgRow) -> Self {
        User::<Uuid>::from_row(row).string()
    }
    pub async fn get_by_uuid(pool: &Pool<Postgres>, uuid: Uuid) -> Result<Self, sqlx::Error> {
        match User::<Uuid>::get_by_uuid(pool, uuid).await {
            Ok(x) => Ok(x.string()),
            Err(x) => Err(x),
        }
    }
    pub async fn get_by_username(
        pool: &Pool<Postgres>,
        username: String,
    ) -> Result<Self, sqlx::Error> {
        match User::<Uuid>::get_by_username(pool, username).await {
            Ok(x) => Ok(x.string()),
            Err(x) => Err(x),
        }
    }
}
impl User<Uuid> {
    pub fn new() -> Self {
        Self::_new(Uuid::nil())
    }
    pub fn string(self) -> User<String> {
        self.map(|x| x.as_hyphenated().to_string())
    }

    pub async fn get_by_uuid(pool: &Pool<Postgres>, uuid: Uuid) -> Result<Self, sqlx::Error> {
        match sqlx::query("SELECT * FROM users WHERE id = $1")
            .bind(uuid)
            .fetch_one(pool)
            .await
        {
            Ok(x) => Ok(User::<Uuid>::from_row(&x)),
            Err(x) => Err(x),
        }
    }
    pub async fn get_by_username(
        pool: &Pool<Postgres>,
        username: String,
    ) -> Result<Self, sqlx::Error> {
        match sqlx::query("SELECT * FROM users WHERE username = $1")
            .bind(username)
            .fetch_one(pool)
            .await
        {
            Ok(x) => Ok(User::<Uuid>::from_row(&x)),
            Err(x) => Err(x),
        }
    }
    pub fn from_row(row: &sqlx::postgres::PgRow) -> Self {
        Self {
            username: row.get("username"),
            id: row.get("id"),
        }
    }
}
