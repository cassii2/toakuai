use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    username: String,
}

#[derive(Serialize, Deserialize)]
struct Word {
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
struct Comment {
    id: u32,
    author: u32,
    parent_word: u32,
    parent_comment: Option<u32>,
    content: String,
}

#[derive(Serialize, Deserialize)]
struct Vote {
    author: u32,
    entry_word: Option<u32>,
    entry_comment: Option<u32>,
    is_upvote: bool,
}

enum ReqType {
    User(User),
    Word(Word),
    Comment(Comment),
    Vote(Vote),
}
