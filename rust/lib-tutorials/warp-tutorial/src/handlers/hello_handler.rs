use warp;

pub async fn hello(name: String) -> Result<impl warp::Reply, warp::Rejection> {
    let reply: String = format!("Hello, {}!", name);
    // println!("{}", &reply);
    Ok(warp::reply::html(reply))
}
