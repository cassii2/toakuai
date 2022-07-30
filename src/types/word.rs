use serde::{Deserialize, Serialize};
use sqlx::Row;
use sqlx::{types::Uuid, Pool, Postgres};
use std::string::String;

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
        self.map(|x| Uuid::parse_str(&x).unwrap())
    }
    pub fn from_row(row: &sqlx::postgres::PgRow) -> Self {
        Word::<Uuid>::from_row(row).string()
    }
}
impl Word<Uuid> {
    pub fn new() -> Self {
        Self::_new(Uuid::nil())
    }
    pub fn string(self) -> Word<String> {
        self.map(|x| x.as_hyphenated().to_string())
    }

    pub async fn get_by_uuid(pool: &Pool<Postgres>, uuid: Uuid) -> Result<Self, sqlx::Error> {
        match sqlx::query("SELECT * FROM words WHERE id = $1")
            .bind(uuid)
            .fetch_one(pool)
            .await
        {
            Ok(x) => Ok(Word::<Uuid>::from_row(&x)),
            Err(x) => Err(x),
        }
    }
    pub fn from_row(row: &sqlx::postgres::PgRow) -> Self {
        Self {
            id: row.get("id"),
            word: row.get("word"),
            author: row.get("author"),
            definition: row.get("definition"),
            forked_from: row.get("forked_from"),
            lang: row.get("lang"),
            gloss: row.get("gloss"),
            frame: row.get("frame"),
            created: row
                .get::<sqlx::types::chrono::NaiveDateTime, _>("created")
                .timestamp(),
            edited: row.get("edited"),
        }
    }
}
