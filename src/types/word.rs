use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, types::Uuid, Pool, Postgres};
use sqlx::{FromRow, Row};
use std::string::String;

use super::comment::Comment;

#[derive(Debug, Serialize, Deserialize)]
pub struct Word<T> {
    pub id: T,
    pub author: Option<T>,
    pub word: String,
    pub definition: String,
    pub forked_from: Option<T>,
    pub lang: String,
    pub gloss: Vec<String>,
    pub frame: Vec<String>,
    pub created: i64, // chrono::Utc.timestamp()
    pub edited: Option<i64>,
}
impl<T: Clone> Word<T> {
    #[inline]
    fn _new(empty: T) -> Self {
        Self {
            id: empty.clone(),
            author: None,
            word: String::new(),
            definition: String::new(),
            forked_from: None,
            lang: String::new(),
            gloss: vec![String::new()],
            frame: vec![String::new()],
            created: 0, // chrono::Utc.timestamp()
            edited: None,
        }
    }
    #[inline]
    fn map<U>(self, f: impl Fn(T) -> U) -> Word<U> {
        Word {
            id: f(self.id),
            author: match self.author {
                Some(x) => Some(f(x)),
                None => None,
            },
            word: self.word,
            definition: self.definition,
            forked_from: match self.forked_from {
                Some(x) => Some(f(x)),
                None => None,
            },
            lang: self.lang,
            gloss: self.gloss,
            frame: self.frame,
            created: self.created,
            edited: self.edited,
        }
    }
}
impl Word<String> {
    pub fn new() -> Self {
        Self::_new(String::new())
    }
    pub fn uuid(self) -> Word<Uuid> {
        self.map(|x| {
            Uuid::parse_str(&x).unwrap_or_else(|e| {
                eprintln!("Could not parse Word UUID: {}\nError: {}", x, e);
                Uuid::nil()
            })
        })
    }
    pub async fn from_uuid(pool: &Pool<Postgres>, uuid: Uuid) -> Result<Self, sqlx::Error> {
        Ok(Word::<Uuid>::from_uuid(pool, uuid).await?.string())
    }
    // pub fn from_row(row: &sqlx::postgres::PgRow) -> Self {
    //     Word::<Uuid>::from_row(row).string()
    // }
    //
    // pub async fn count_votes(&self, pool: &Pool<Postgres>) -> Result<(u64, u64), sqlx::Error> {
    //     let mut votes = sqlx::query("SELECT * FROM votes WHERE entry_word = $1")
    //         .bind(self.id.clone())
    //         .fetch(pool);
    //     let mut up = 0;
    //     let mut down = 0;
    //     while let Some(row) = &votes.try_next().await? {
    //         if row.get::<bool, _>("is_upvote") == true {
    //             up += 1;
    //         } else {
    //             down += 1;
    //         }
    //     }
    //     Ok((up, down))
    // }
}
impl Word<Uuid> {
    pub fn new() -> Self {
        Self::_new(Uuid::nil())
    }
    pub fn string(self) -> Word<String> {
        self.map(|x| x.as_hyphenated().to_string())
    }

    pub async fn from_uuid(pool: &Pool<Postgres>, uuid: Uuid) -> Result<Self, sqlx::Error> {
        match sqlx::query("SELECT * FROM words WHERE id = $1")
            .bind(uuid)
            .fetch_one(pool)
            .await
        {
            Ok(x) => Ok(Word::<Uuid>::from_row(&x)?),
            Err(x) => Err(x),
        }
    }
    pub fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            word: row.try_get("word")?,
            author: row.try_get("author")?,
            definition: row.try_get("definition")?,
            forked_from: row.try_get("forked_from")?,
            lang: row.try_get("lang")?,
            gloss: row.try_get("gloss")?,
            frame: row.try_get("frame")?,
            created: row
                .try_get::<sqlx::types::chrono::NaiveDateTime, _>("created")?
                .timestamp(),
            edited: row.try_get("edited")?,
        })
    }

    // returns (upvotes, downvotes)
    pub async fn count_votes(&self, pool: &Pool<Postgres>) -> Result<(u64, u64), sqlx::Error> {
        let mut votes = sqlx::query("SELECT * FROM votes WHERE entry_word = $1")
            .bind(self.id)
            .fetch(pool);
        let mut up = 0;
        let mut down = 0;
        while let Some(row) = &votes.try_next().await? {
            if row.get::<bool, _>("is_upvote") == true {
                up += 1;
            } else {
                down += 1;
            }
        }
        Ok((up, down))
    }
    pub async fn get_comments(
        &self,
        pool: &Pool<Postgres>,
    ) -> Result<Vec<Comment<Uuid>>, sqlx::Error> {
        let mut rows = sqlx::query("SELECT * FROM comments WHERE parent_word = $1")
            .bind(self.id)
            .fetch(pool);
        let mut ret = Vec::<Comment<Uuid>>::new();
        while let Some(row) = &rows.try_next().await? {
            let comment = Comment::<Uuid>::from_row(row);
            ret.push(comment);
        }
        return Ok(ret);
    }
}

impl<'r> FromRow<'r, PgRow> for Word<Uuid> {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Word::<Uuid>::from_row(row)
    }
}
