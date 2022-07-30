use serde::{Deserialize, Serialize};
use sqlx::{types::Uuid, Pool, Postgres, Row};
use std::string::String;

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
        self.map(|x| {
            Uuid::parse_str(&x).unwrap_or_else(|e| {
                eprintln!("Could not parse Vote UUID: {}\nError: {}", x, e);
                Uuid::nil()
            })
        })
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
