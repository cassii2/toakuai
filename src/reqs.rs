use std::result::Result;

use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, Connection};

#[derive(Serialize, Deserialize)]
pub struct User {
    id: u32,
    username: String,
}

#[derive(Serialize, Deserialize)]
pub struct Word {
    id: u32,
    author: u32,
    word: String,
    definition: String,
    forked_from: Option<u32>,
    lang: [char; 8],
    gloss: [String; 1],
    frame: [[char; 3]; 1],
    created: i64, // chrono::Utc.timestamp()
    edited: Option<i64>,
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
        .connect("postgres://postgres@localhost/toaq")
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
