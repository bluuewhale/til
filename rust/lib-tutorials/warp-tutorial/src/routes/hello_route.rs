// It is equal to use
// let hello = path!("hello" / String);
// in main.rs file.

use warp::{filters::BoxedFilter, Filter};

// 1. "hello"
fn path_prefix() -> BoxedFilter<()> {
    warp::path("hello").boxed()
}

// 2. / String
pub fn hello() -> BoxedFilter<(String,)> {
    warp::get() // 3.
        .and(path_prefix()) // 4.
        .and(warp::path::param::<String>()) // 5.
        .boxed()
}
