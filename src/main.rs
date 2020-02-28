use log::Level;
use warp::{Filter, Rejection};

pub mod datatypes;
mod sql;

use datatypes::*;
use std::ops::Deref;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

use http::StatusCode;
use log::{error, info};
use reqwest::Error;
use rusqlite::{params, Connection};
use std::borrow::Borrow;
use std::fmt::Debug;
use std::future::Future;
use std::time::Duration;
use warp::filters::ws::WebSocket;
use warp::reject::Reject;
use warp::reply::Json;

use futures_util::{sink::SinkExt, stream::StreamExt};
use warp::ws::Message;

#[derive(Debug, Clone)]
struct ServerError<E: 'static + std::error::Error + Sync + Send + Debug> {
    error: E,
}

impl<E: 'static + std::error::Error + Sync + Send + Debug> Reject for ServerError<E> {}
impl<E: 'static + std::error::Error + Sync + Send + Debug> From<E> for ServerError<E> {
    fn from(error: E) -> Self {
        ServerError { error }
    }
}
impl<E: 'static + std::error::Error + Sync + Send + Debug> Into<warp::reject::Rejection>
    for ServerError<E>
{
    fn into(self) -> Rejection {
        warp::reject::custom(self)
    }
}

#[tokio::main]
async fn main() -> rusqlite::Result<()> {
    // Setup the loggers
    simple_logger::init_with_level(Level::Info).unwrap();
    let log = warp::log("web");

    // Setup the database
    let db_conn = sql::get_db_conn()?;
    sql::db_setup(&db_conn);
    let db_conn_filter = warp::any()
      .map(|| sql::db_conn());

    let state = Arc::new(Mutex::new(App::default()));
    let state_filter = warp::any()
      .map(move || Arc::clone(&state));

    // GET /<anything>
    let static_content_route = warp::any()
      .and(warp::get())
      .and(warp::fs::dir("web/dist"));

    // GET /api/state
    let get_app_state_route = warp::get()
      .and(db_conn_filter.clone())
      .and_then(get_state);

    // POST /api/state/client/new
    let new_client_route = warp::path!("client" / "new")
      .and(warp::post())
      .and(db_conn_filter.clone())
      .and(warp::body::json::<NatsClient>())
      .and_then(handle_insert_client);

    // POST /api/state/client/update
    let update_client_route = warp::path!("client" / "update")
      .and(warp::post())
      .and(db_conn_filter.clone())
      .and(warp::body::json::<NatsClient>())
      .and_then(handle_update_client);

    // GET /api/state/client/delete/<id>
    let delete_client_route = warp::path!("client" / "delete" / i64 )
      .and(warp::get())
      .and(db_conn_filter.clone())
      .and_then(handle_delete_client);

    // POST /api/state/server/new
    let new_server_route = warp::path!("server" / "new")
      .and(warp::post())
      .and(db_conn_filter.clone())
      .and(warp::body::json::<NatsServer>())
      .and_then(handle_insert_server);

    // POST /api/state/server/update
    let update_server_route = warp::path!("server" / "update")
      .and(warp::post())
      .and(db_conn_filter.clone())
      .and(warp::body::json::<NatsServer>())
      .and_then(handle_update_server);

    // GET /api/state/server/delete/<id>
    let delete_server_route = warp::path!("server" / "delete" / i64 )
      .and(warp::get())
      .and(db_conn_filter.clone())
      .and_then(handle_delete_server);

    // GET /client with websocket upgrade
    let client_subscribe_route = warp::path!("client" / i64)
        .and(warp::ws())
        .map(|client_id: i64, ws: warp::ws::Ws| ws.on_upgrade(handle_client_subscription));

    let api_route = warp::path("api")
        .and(warp::path("state"))
        .and(get_app_state_route
          .or(new_client_route)
          .or(update_client_route)
          .or(delete_client_route)
          .or(new_server_route)
          .or(update_server_route)
          .or(delete_server_route)
        );

    let route = static_content_route
        .or(api_route)
        .or(client_subscribe_route)
        .with(log);

    warp::serve(route).run(([0, 0, 0, 0], 80)).await;

    Ok(())
}

async fn get_state(conn: Connection) -> Result<impl warp::Reply, warp::Rejection> {
    let mut st = App::default();
    let mut svs = match sql::get_servers(&conn) {
        Ok(v) => v,
        Err(e) => return Err(ServerError::from(e).into()),
    };
    let builder = reqwest::ClientBuilder::new().connect_timeout(Duration::new(0, 250_000_000));
    let cl = builder.build().unwrap();
    for s in svs.iter_mut() {
        if let Err(e) = s.get_varz(&cl).await {
            error!("{:?}", e);
        }
    }
    let mut cls = match sql::get_clients(&conn) {
        Ok(v) => v,
        Err(e) => return Err(ServerError::from(e).into()),
    };
    st.set_servers(svs);
    st.set_clients(cls);
    info!("{:?}", st);
    Ok(warp::reply::json(&st.clone()))
}

async fn handle_client_subscription(mut ws: WebSocket) {
    let address = "192.168.1.202:4222".parse().unwrap();
    let client = rants::Client::new(vec![address]);
    let subject = "yahoo-finance.>".parse().unwrap();

    client.connect_mut().await.echo(true);
    client.connect().await;

    let (_, mut recv) = client.subscribe(&subject, 1_048_576).await.unwrap();

    ws.send_all(
        &mut recv.map(|msg| Ok(Message::text(std::str::from_utf8(msg.payload()).unwrap()))),
    )
    .await;
}

async fn handle_insert_client(conn: Connection, client: NatsClient) -> Result<impl warp::Reply, warp::Rejection> {
    match sql::insert_client(&conn, client) {
        Ok(_) => Ok(warp::reply()),
        Err(e) => Err(ServerError::from(e).into()),
    }
}

async fn handle_update_client(conn: Connection, client: NatsClient) -> Result<impl warp::Reply, warp::Rejection> {
    match sql::update_client(&conn, client) {
        Ok(_) => Ok(warp::reply()),
        Err(e) => Err(ServerError::from(e).into())
    }
}

async fn handle_delete_client(client_id: i64, conn: Connection) -> Result<impl warp::Reply, warp::Rejection> {
    match sql::delete_client(&conn, client_id) {
        Ok(_) => Ok(warp::reply()),
        Err(e) => Err(ServerError::from(e).into()),
    }
}

async fn handle_insert_server(conn: Connection, server: NatsServer) -> Result<impl warp::Reply, warp::Rejection> {
    match sql::insert_server(&conn, server) {
        Ok(_) => Ok(warp::reply()),
        Err(e) => Err(ServerError::from(e).into()),
    }
}

async fn handle_update_server(conn: Connection, server: NatsServer) -> Result<impl warp::Reply, warp::Rejection> {
    match sql::update_server(&conn, server) {
        Ok(_) => Ok(warp::reply()),
        Err(e) => Err(ServerError::from(e).into()),
    }
}

async fn handle_delete_server(server_id: i64, conn: Connection) -> Result<impl warp::Reply, warp::Rejection> {
    match sql::delete_server(&conn, server_id) {
        Ok(_) => Ok(warp::reply()),
        Err(e) => Err(ServerError::from(e).into()),
    }
}