use std::result::Result;

use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};
use sqlx::postgres::PgPoolOptions;
use sqlx::types::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String, //u128 hyphenated
    pub username: String,
}
impl User {
    pub fn new() -> Self {
        Self {
            id: String::new(),
            username: String::new(),
        }
    }
}

// Can't derive serialize from serde, when using UUIDs. So we use String instead
#[derive(Debug, Serialize, Deserialize)]
pub struct Word {
    id: String,     //UUID
    author: String, //UUID
    word: String,
    definition: String,
    forked_from: Option<String>, //UUID
    lang: [char; 8],
    gloss: [String; 1],
    frame: [[char; 3]; 1],
    created: i64, // chrono::Utc.timestamp()
    edited: Option<i64>,
}
impl Word {
    pub fn new() -> Self {
        Self {
            id: String::new(),
            author: String::new(),
            word: String::new(),
            definition: String::new(),
            forked_from: None,
            lang: [' '; 8],
            gloss: [String::new(); 1],
            frame: [[' '; 3]; 1],
            created: 0, // chrono::Utc.timestamp()
            edited: None,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Comment {
    id: u32,
    author: u32,
    parent_word: u32,
    parent_comment: Option<u32>,
    content: String,
}

#[derive(Serialize, Deserialize)]
pub struct Vote {
    author: u32,
    entry_word: Option<u32>,
    entry_comment: Option<u32>,
    is_upvote: bool,
}

pub async fn init_sql() -> Result<(), sqlx::Error> {
    // This project uses PostgreSQL
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://toakuai@localhost/toaq")
        .await?;
    todo!()
}

pub enum Reqs {
    User(User),
    Word(Word),
    Comment(Comment),
    Vote(Vote),
}
pub enum ReqType {
    User,
    Word,
    Comment,
    Vote,
}

// Get from SQL database
pub async fn get(request_type: ReqType, id: u32) -> Reqs {
    match request_type {
        ReqType::User => todo!(),
        ReqType::Word => todo!(),
        ReqType::Comment => todo!(),
        ReqType::Vote => todo!(),
    }
}

// Return whether it worked or not
pub async fn set(request_data: Reqs) -> bool {
    todo!()
}
