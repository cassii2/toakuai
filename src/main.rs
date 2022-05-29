use std::{convert::Infallible, time::Duration};

use serde::{Deserialize, Serialize};
use tokio::time::interval;
use tokio_stream::{wrappers::IntervalStream, StreamExt};
use warp::{sse, Filter};

#[derive(Deserialize, Serialize)]
struct Myreq {
    name: String,
    num: u32,
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let hello = warp::path("hello").map(|| {
        let mut counter: u64 = 0;
        let interval = interval(Duration::from_secs(1));
        let stream = IntervalStream::new(interval);
        let event_stream = stream.map(move |_| {
            counter += 1;
            sse_counter(counter)
        });
        warp::sse::reply(event_stream)
    });
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

    let routes_get = warp::get().and(hello.or(again).or(root));
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

fn sse_counter(counter: u64) -> Result<sse::Event, Infallible> {
    Ok(sse::Event::default().data(counter.to_string()))
}
