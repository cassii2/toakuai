use std::process::exit;

use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use sqlx::types::Uuid;
use sqlx::{Pool, Postgres, Row};

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

#[derive(Debug, Serialize, Deserialize)]
pub struct Vote<T> {
    author: T,
    entry_word: Option<T>,
    entry_comment: Option<T>,
    is_upvote: bool,
}
impl<T> Vote<T> {
    #[inline]
    fn _new(empty: T) -> Self {
        Self {
            author: empty,
            entry_word: None,
            entry_comment: None,
            is_upvote: false,
        }
    }

    #[inline]
    fn map<U>(self, f: impl Fn(T) -> U) -> Vote<U> {
        Vote {
            author: f(self.author),
            entry_word: match self.entry_word {
                Some(x) => Some(f(x)),
                None => None,
            },
            entry_comment: match self.entry_comment {
                Some(x) => Some(f(x)),
                None => None,
            },
            is_upvote: self.is_upvote,
        }
    }

    pub fn get_name() -> String {
        String::from("vote")
    }
}
impl Vote<String> {
    pub fn new() -> Self {
        Self::_new(String::new())
    }
    pub fn uuid(self) -> Vote<Uuid> {
        self.map(|x| Uuid::parse_str(&x).unwrap())
    }
}
impl Vote<Uuid> {
    pub fn new() -> Self {
        Self::_new(Uuid::nil())
    }
    pub fn string(self) -> Vote<String> {
        self.map(|x| x.as_hyphenated().to_string())
    }
}

pub enum ToakuaiReq<T> {
    User(User<T>),
    Word(Word<T>),
    Comment(Comment<T>),
    Vote(Vote<T>),
}

pub async fn init_sql() -> sqlx::Pool<sqlx::Postgres> {
    // This project uses PostgreSQL
    match PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://toakuai@localhost/toaq")
        .await
    {
        Err(x) => {
            println!("Error connecting to SQL Server!\n{}", x);
            exit(-1);
        }
        Ok(x) => x,
    }
}
