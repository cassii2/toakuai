mod sql;
mod types;

use crate::sql::init_sql;
use crate::types::{comment::Comment, user::User, vote::Vote, word::Word};
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use sqlx::{types::Uuid, Row};
use warp::Filter;

const MYPORT: u16 = 3000;

#[derive(Deserialize, Serialize)]
struct Myreq {
    name: String,
    num: u32,
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let pool = init_sql().await;

    // let mut rows = sqlx::query("SELECT * FROM users").fetch(&pool);
    // while let Some(row) = &rows.try_next().await.unwrap() {
    //     let user = User::<Uuid>::from_row(row);
    //     println!("{:?}", user);
    // }

    // let mut words = sqlx::query("SELECT * FROM words").fetch(&pool);
    // while let Some(row) = &words.try_next().await.unwrap() {
    //     let word = Word::<Uuid>::from_row(row).unwrap();
    //     let (up, _) = word.count_votes(&pool).await.unwrap();
    //     println!("UUID: {:?}\nVotes: {}", word, up);
    // }

    loop {
        let mut buffer = String::new();
        let mut stdin = std::io::stdin();
        println!("Select from list:");
        println!("1: Get");
        println!("2: Add");
        println!("3: Delete");
        println!("\n0: Exit");
        stdin.read_line(&mut buffer).unwrap();
        buffer = buffer.trim().to_string();

        match buffer.parse::<u32>().unwrap_or_else(|_| {
            println!("Could not parse \"{}\"", buffer);
            std::process::exit(1);
        }) {
            0 => {
                break;
            }
            1 => {
                println!("Get what?");
                println!("1. Word");
                println!("2. Comment");
                buffer = String::new();
                stdin.read_line(&mut buffer).unwrap();
                buffer = buffer.trim().to_string();
                println!("Buffer is: {}", buffer);
                match buffer.parse::<i32>().unwrap() {
                    1 => {
                        println!("Enter word: ");
                        buffer = String::new();
                        stdin.read_line(&mut buffer).unwrap();
                        buffer = buffer.trim().to_string();
                        let word = Word::<Uuid>::from_row(
                            &sqlx::query("SELECT * FROM words WHERE word = $1")
                                .bind(buffer.clone())
                                .fetch_one(&pool)
                                .await
                                .unwrap(),
                        )
                        .unwrap();
                        println!("{:?}", word);
                    }
                    2 => {
                        println!("Enter author: ");
                        buffer = String::new();
                        stdin.read_line(&mut buffer).unwrap();
                        buffer = buffer.trim().to_string();
                        let comment = Comment::<Uuid>::from_row(
                            &sqlx::query("SELECT * FROM comments WHERE author = (SELECT id FROM users WHERE username = $1)")
                                .bind(buffer.clone())
                                .fetch_one(&pool)
                                .await
                                .unwrap(),
                        );
                        println!("{:?}", comment);
                    }
                    _ => {
                        continue;
                    }
                }
            }
            _ => {
                continue;
            }
        }
    }

    // let again = warp::path("again")
    //     .map(|| println!("Logging"))
    //     .untuple_one()
    //     .and(warp::path::param())
    //     .and(warp::header("user-agent"))
    //     .then(|param: String, agent: String| async move {
    //         format!("Hello {}, on user agent {}", param, agent)
    //     });
    // let root = warp::path::end().map(|| format!("Hello, welcome to root!"));

    // let api = warp::path("api")
    //     .and(warp::body::content_length_limit(1024 * 16))
    //     .and(warp::body::json())
    //     .map(|mut jsonreq: Myreq| {
    //         jsonreq.num = 25;
    //         warp::reply::json(&jsonreq)
    //     });

    // let routes_get = warp::get().and(again.or(root));
    // let routes_post = warp::post().and(api);

    // let _ = tokio::join!(
    //     tokio::spawn(warp::serve(routes_all).run(([127, 0, 0, 1], MYPORT))),
    //     // tokio::spawn(
    //     //     warp::serve(routes_all)
    //     //         .tls()
    //     //         .cert_path("tls/cert.pem")
    //     //         .key_path("tls/kry.rsa")
    //     //         .run(([127, 0, 0, 1], 3443))
    //     // )
    // );
}

fn initserver() {
    let api = warp::post().and(
        warp::path("api")
            .and(warp::body::content_length_limit(1024 * 16))
            .and(warp::body::json())
            .map(|mut jsonreq: Myreq| {
                jsonreq.num = 25;
                warp::reply::json(&jsonreq)
            }),
    );
    let index = warp::get()
        .and(warp::path::end().or(warp::path("index.html")))
        .map(|_| format!(""));

    let all = api.or(index).boxed();

    //TODO: make this return something so that we can run the server at some point
    todo!()
}
