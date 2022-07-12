mod reqs;

use std::process::exit;

use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::{PgPoolOptions, PgRow},
    Row,
};
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

    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://toakuai@localhost/toaq")
        .await
    {
        Ok(x) => x,
        Err(_) => {
            println!("Error!!");
            exit(-1);
        }
    };

    // let row: (sqlx::types::Uuid, String) = sqlx::query_as("SELECT * FROM users")
    //     .fetch_one(&pool)
    //     .await
    //     .unwrap();
    // println!("UUID: {}, Username: {}", row.0, row.1);
    let mut user = reqs::User {
        id: String::new(),
        username: String::new(),
    };

    let mut rows = sqlx::query("SELECT * FROM users")
        .map(move |row: PgRow| {
            user.id = row.get(0).hyphenated().to_string();
            user.username = row.get(1);
            println!("{}, {}", user.id, user.username);
        })
        .fetch(&pool);
    while let Some(_) = rows.try_next().await.unwrap() {}

    // while let Some(row) = stream.try_next().await.unwrap() {
    //     println!("ID: {}, Username: {}", row.id, row.username);
    // }

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

    // let routes_all = routes_get.or(routes_post);

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
