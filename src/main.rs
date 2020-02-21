use warp::Filter;
use log::Level;

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(Level::Info).unwrap();
    let log = warp::log("web");

    let static_content_route = warp::any()
      .and(warp::fs::dir("web\\dist"))
      .with(log);

    let api_route = warp::path("api")
      .map(|| "API Test");

    let route = static_content_route.or(api_route);

    warp::serve(route)
      .run(([0, 0, 0, 0], 80))
      .await;
}