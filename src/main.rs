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

    let static_content_route = warp::any()
      .and(warp::get())
      .and(warp::fs::dir("web/dist"))
      .with(log);

    let app_state_route = warp::path("state")
      .and(warp::get())
      .and(state_filter.clone())
      .and_then(get_state);

    // let client_subscribe_route = warp::path("client")
    //   .

    let api_route = warp::path("api")
      .and(
          app_state_route
      );

    let route = static_content_route.or(api_route);

    warp::serve(route)
      .run(([0, 0, 0, 0], 80))
      .await;

    Ok(())
}

async fn get_state(state: Arc<Mutex<App>>) -> Result<impl warp::Reply, warp::Rejection> {
    let mut st = state.lock().await;
    let conn = sql::db_conn();
    let mut svs = sql::get_servers(&conn).map_err(|_| warp::reject::not_found())?;
    let builder = reqwest::ClientBuilder::new()
      .connect_timeout(Duration::new(0, 250_000_000));
    let cl = builder.build().unwrap();
    for s in svs.iter_mut() {
        if let Err(e) = s.get_varz(&cl).await {
            error!("{:?}", e);
        }
    }
    let mut cls = sql::get_clients(&conn).map_err(|_| warp::reject::not_found())?;
    st.set_servers(svs);
    st.set_clients(cls);
    Ok(warp::reply::json(&st.clone()))
}

// async fn handle_get_state() -> impl Future<Output = Result<Json, Rejection>> {
//     async {
//         let mut state = App::default();
//         let conn = sql::db_conn();
//         let mut svs = sql::get_servers(&conn).map_err(|_| warp::reject::not_found())?;
//         for mut s in &mut svs {
//             s.varz = match s.get_varz().await {
//                 Ok(v) => Some(v),
//                 Err(e) => {
//                     error!("{}", e);
//                     None
//                 },
//             }
//         }
//         state.servers = svs;
//         Ok(warp::reply::json(&state))
//     }
// }

