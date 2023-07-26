use warp::{Filter, Rejection, Reply};
use std::env;
use serde::{Deserialize, Serialize};


async fn hello (name: String) -> Result<impl Reply, Rejection> {
    Ok(format!("Hello, World! {}", name))
}

#[derive(Deserialize, Serialize)]
struct Employee {
    name: String,
    rate: u32,
}

#[tokio::main]
async fn main() {

    // POST /employees/:rate  {"name":"Sean","rate":2}
    let promote = warp::post()
        .and(warp::path("employees"))
        .and(warp::path::param::<u32>())
        // Only accept bodies smaller than 16kb...
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .map(|rate, mut employee: Employee| {
            employee.rate = rate;
            warp::reply::json(&employee)
        });

    let numbers = warp::path!("numbers" / u32 / ..)
        .map(|id| format!("The number is {}", id));

    let hello = warp::path!("hello" / String / ..)
        .and_then(hello);

    warp::serve(numbers.or(hello).or(promote))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
