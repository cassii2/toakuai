mod paths;
mod reqs;

use paths::AllPaths;
use serde::{Deserialize, Serialize};
use warp::{filters::BoxedFilter, Filter};

#[derive(Deserialize, Serialize)]
struct Myreq {
    name: String,
    num: u32,
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let again = warp::path("again")
        .map(|| println!("Logging"))
        .untuple_one()
        .and(warp::path::param())
        .and(warp::header("user-agent"))
        .then(|param: String, agent: String| async move {
            format!("Hello {}, on user agent {}", param, agent)
        });
    let root = warp::path::end().map(|| format!("Hello, welcome to root!"));

    let api = warp::path("api")
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .map(|mut jsonreq: Myreq| {
            jsonreq.num = 25;
            warp::reply::json(&jsonreq)
        });

    let routes_get = warp::get().and(again.or(root));
    let routes_post = warp::post().and(api);

    let routes_all = routes_get.or(routes_post);

    let _ = tokio::join!(
        tokio::spawn(warp::serve(routes_all).run(([127, 0, 0, 1], 3000))),
        // tokio::spawn(
        //     warp::serve(routes_all)
        //         .tls()
        //         .cert_path("tls/cert.pem")
        //         .key_path("tls/kry.rsa")
        //         .run(([127, 0, 0, 1], 3443))
        // )
    );
}

fn init() -> AllPaths<BoxedFilter<(String,)>> {
    let api = warp::path("api")
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .map(|mut jsonreq: Myreq| {
            jsonreq.num = 25;
            warp::reply::json(&jsonreq)
        });
    // AllPaths::<(String,)>::new(warp::get().map(|| format!("")).boxed())
    todo!()
}
