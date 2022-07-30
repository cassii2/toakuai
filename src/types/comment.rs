use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use sqlx::{types::Uuid, Pool, Postgres, Row};
use std::string::String;

#[derive(Debug, Serialize, Deserialize)]
pub struct Comment<T> {
    id: T,
    author: T,
    parent_word: T,
    parent_comment: Option<T>,
    content: String,
}
impl<T: Clone> Comment<T> {
    #[inline]
    fn _new(empty: T) -> Self {
        Self {
            id: empty.clone(),
            author: empty.clone(),
            parent_word: empty,
            parent_comment: None,
            content: String::new(),
        }
    }

    #[inline]
    fn map<U>(self, f: impl Fn(T) -> U) -> Comment<U> {
        Comment {
            id: f(self.id),
            author: f(self.author),
            parent_word: f(self.parent_word),
            parent_comment: match self.parent_comment {
                Some(x) => Some(f(x)),
                None => None,
            },
            content: self.content,
        }
    }
}
impl Comment<String> {
    pub fn new() -> Self {
        Self::_new(String::new())
    }
    pub fn uuid(self) -> Comment<Uuid> {
        self.map(|x| Uuid::parse_str(&x).unwrap())
    }
}
impl Comment<Uuid> {
    pub fn new() -> Self {
        Self::_new(Uuid::nil())
    }
    pub fn string(self) -> Comment<String> {
        self.map(|x| x.as_hyphenated().to_string())
    }
    pub fn from_row(row: &sqlx::postgres::PgRow) -> Self {
        Self {
            id: row.get("id"),
            author: row.get("author"),
            parent_word: row.get("parent_word"),
            parent_comment: row.get("parent_comment"),
            content: row.get("content"),
        }
    }
    pub async fn from_uuid(pool: &Pool<Postgres>, uuid: Uuid) -> Result<Self, sqlx::Error> {
        match sqlx::query("SELECT * FROM comments WHERE id = $1")
            .bind(uuid)
            .fetch_one(pool)
            .await
        {
            Ok(x) => Ok(Comment::<Uuid>::from_row(&x)),
            Err(x) => Err(x),
        }
    }
    pub async fn from_word(pool: &Pool<Postgres>, word: Uuid) -> Result<Vec<Self>, sqlx::Error> {
        // todo
        let mut ret = Vec::<Comment<Uuid>>::new();
        let mut query = sqlx::query("SELECT * FROM comments WHERE parent_word = $1")
            .bind(word)
            .fetch(pool);
        while let Some(row) = &query.try_next().await? {
            ret.push(Comment::<Uuid>::from_row(row));
        }

        return Ok(ret);
    }
}
