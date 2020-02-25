use warp::{Filter, Rejection};
use log::Level;

pub mod datatypes;
mod sql;

use datatypes::*;
use std::sync::{Arc};
use std::ops::Deref;
use tokio::sync::{RwLock, Mutex};

use rusqlite::{params, Connection};
use warp::reply::Json;
use log::{info, error};
use reqwest::Error;
use std::future::Future;
use std::borrow::Borrow;
use std::time::Duration;
use http::StatusCode;

#[tokio::main]
async fn main() -> rusqlite::Result<()> {
    // Setup the loggers
    simple_logger::init_with_level(Level::Info).unwrap();
    let log = warp::log("web");

    // Setup the database
    let db_conn = sql::get_db_conn()?;
    sql::db_setup(&db_conn);

    let state = Arc::new(Mutex::new(App::default()));
    let state_filter = warp::any().map(move || Arc::clone(&state));

    // GET /<anything>
    let static_content_route = warp::any()
      .and(warp::get())
      .and(warp::fs::dir("web/dist"))
      .with(log);

    // GET /api/state
    let get_app_state_route = warp::get()
      .and(state_filter.clone())
      .and_then(get_state);

    // POST /api/state
    let post_app_state_route = warp::post()
      .and(state_filter.clone())
      .and(warp::body::json::<App>())
      .and_then(save_state);

    // let client_subscribe_route = warp::path("client")
    //   .

    let api_route = warp::path("api")
      .and(warp::path("state"))
      .and(get_app_state_route.or(post_app_state_route));

    let route = static_content_route.or(api_route);

    warp::serve(route)
      .run(([0, 0, 0, 0], 80))
      .await;

    Ok(())
}

async fn get_state(state: Arc<Mutex<App>>) -> Result<Box<dyn warp::Reply> , warp::Rejection> {
    let mut st = state.lock().await;
    let conn = sql::db_conn();
    let mut svs = match sql::get_servers(&conn) {
        Ok(v) => v,
        Err(e) => return Ok(Box::new(warp::reply::with_status(format!("{:?}", e), StatusCode::INTERNAL_SERVER_ERROR))),
    };
    let builder = reqwest::ClientBuilder::new()
      .connect_timeout(Duration::new(0, 250_000_000));
    let cl = builder.build().unwrap();
    for s in svs.iter_mut() {
        if let Err(e) = s.get_varz(&cl).await {
            error!("{:?}", e);
        }
    }
    let mut cls = match sql::get_clients(&conn) {
        Ok(v) => v,
        Err(e) => return Ok(Box::new(warp::reply::with_status(format!("{:?}", e), StatusCode::INTERNAL_SERVER_ERROR))),
    };
    st.set_servers(svs);
    st.set_clients(cls);
    info!("{:?}", st);
    Ok(Box::new(warp::reply::json(&st.clone())))
}

async fn save_state(state: Arc<Mutex<App>>, new_state: App) -> Result<impl warp::Reply, warp::Rejection> {
    let mut st = state.lock().await;
    *st = new_state;
    Ok(warp::reply::reply())
}


