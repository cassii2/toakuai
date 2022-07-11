use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    username: String,
}

#[derive(Serialize, Deserialize)]
struct Word {
    id: u32,
    author: User,
    word: String,
    definition: String,
    forked_from: Option<Box<Word>>,
    lang: [char; 8],
    gloss: [String; 1],
    frame: [[char; 3]; 1],
    created: i64, // chrono::Utc.timestamp()
    edited: Option<i64>,
}

#[derive(Serialize, Deserialize)]
struct Comment {
    id: u32,
    author: User,
    parent_word: Word,
    parent_comment: Option<Box<Comment>>,
    content: String,
}

#[derive(Serialize, Deserialize)]
struct Vote {
    author: User,
    entry_word: Option<Word>,
    entry_comment: Option<Comment>,
    is_upvote: bool,
}
