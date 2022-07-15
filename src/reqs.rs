use std::process::exit;

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
            Ok(x) => Ok(Self {
                id: x.get("id"),
                username: x.get("username"),
            }),
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
            Ok(x) => Ok(Self {
                id: x.get("id"),
                username: x.get("username"),
            }),
            Err(x) => Err(x),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Word<T> {
    id: T,
    author: T,
    word: String,
    definition: String,
    forked_from: Option<T>,
    lang: [char; 8],
    gloss: Vec<String>,
    frame: Vec<[char; 3]>,
    created: i64, // chrono::Utc.timestamp()
    edited: Option<i64>,
}
impl<T: Clone> Word<T> {
    #[inline]
    fn _new(empty: T) -> Self {
        Self {
            id: empty.clone(),
            author: empty,
            word: String::new(),
            definition: String::new(),
            forked_from: None,
            lang: [' '; 8],
            gloss: vec![String::new()],
            frame: vec![[' '; 3]],
            created: 0, // chrono::Utc.timestamp()
            edited: None,
        }
    }
    #[inline]
    fn map<U>(self, f: impl Fn(T) -> U) -> Word<U> {
        Word {
            id: f(self.id),
            author: f(self.author),
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
}
impl Word<Uuid> {
    fn new() -> Self {
        Self::_new(Uuid::nil())
    }
    pub fn string(self) -> Word<String> {
        self.map(|x| x.as_hyphenated().to_string())
    }

    pub async fn get_by_uuid(pool: &Pool<Postgres>, uuid: Uuid) -> Result<Self, sqlx::Error> {
        match sqlx::query("SELECT * FROM users WHERE id = $1")
            .bind(uuid)
            .fetch_one(pool)
            .await
        {
            Ok(x) => Ok(Self {
                id: x.get("id"),
                author: x.get("username"),
                word: x.get("word"),
                definition: x.get("definition"),
                forked_from: x.get("forked_from"),
                lang: todo!(),
                gloss: todo!(),
                frame: todo!(),
                created: todo!(),
                edited: todo!(),
            }),
            Err(x) => Err(x),
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

    pub fn get_name() -> String {
        String::from("comment")
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
