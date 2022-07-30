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
    //     let word = Word::<Uuid>::from_row(row);
    //     println!("UUID: {:?}", word);
    // }

    let usertest = User::<Uuid>::from_username(&pool, String::from("main"));
    match usertest.await {
        Ok(x) => println!("User: {:?}", x),
        Err(x) => println!("Can't find user\nError: {}", x),
    }
    let usertest = User::<Uuid>::from_username(&pool, String::from("invalid"));
    match usertest.await {
        Ok(x) => println!("User: {:?}", x),
        Err(x) => println!("Can't find user\nError: {}", x),
    }

    let mut wronguser = User::<String>::new();
    wronguser.id = "a".to_string();
    println!("{:?}", wronguser.uuid());

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
