use warp::{self, Filter};

use console::Style;

mod api;
mod handlers; // 2.
mod routes; // 1. // 3.

use self::{handlers::hello_handler, routes::hello_route};

// It will only work with $cargo test
// For example, $cargo test hello -- --nocapture
// #[cfg(test)]
// mod tests;

#[tokio::main]
async fn main() {
    let target: String = "0.0.0.0:8000".parse().unwrap();
    let blue = Style::new().blue();

    // 4.
    let end = hello!().with(warp::log("hello"));

    println!("\nRust Warp Server ready at {}", blue.apply_to(&target));
    println!("Use $curl 0.0.0.0:8000/hello/www.steadylearner.com to test the end point.");

    warp::serve(end).run(([0, 0, 0, 0], 8000)).await;
}
