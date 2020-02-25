use serde::{Serialize, Deserialize};
use reqwest;
use std::error::Error;
use log::info;
use warp::reply::Response;

#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct App {
    pub servers: Vec<NatsServer>,
    pub clients: Vec<NatsClient>
}

impl App {
    pub fn set_servers(&mut self, servers: Vec<NatsServer>) {
        self.servers = servers;
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NatsServer {
    pub id: i64,
    pub name: String,
    pub host: String,
    pub port: u16,
    pub monitoring_port: u16,
    pub varz: Option<ServerVarz>,
    pub subjects: Vec<SubjectTreeNode>,
    pub publications: Vec<Publication>
}

impl NatsServer {
    pub async fn get_varz(&mut self, cl: &reqwest::Client) -> reqwest::Result<()> {
        let varz = cl.get(&format!("http://{}:{}/varz", self.host, self.monitoring_port))
          .send()
          .await?
          .json::<ServerVarz>()
          .await?;
        info!("{:?}", varz);
        self.varz = Some(varz);
        Ok(())
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SubjectTreeNode {
    subject_str: String,
    subjects: Vec<SubjectTreeNode>
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Publication {
    subject: String,
    message: String
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ServerVarz {
    server_id: String,
    server_name: String,
    version: String,
    proto: u64,
    git_commit: String,
    go: String,
    host: String,
    port: u16,
    max_connections: u64,
    ping_interval: u64,
    ping_max: u64,
    http_host: String,
    http_port: u16,
    https_port: u16,
    auth_timeout: u64,
    max_control_line: u64,
    max_payload: u64,
    max_pending: u64,
    cluster: ServerCluster,
    tls_timeout: f64,
    write_deadline: u64,
    start: String,
    now: String,
    uptime: String,
    mem: u64,
    cores: u64,
    cpu: u8,
    connections: u64,
    total_connections: u64,
    routes: u64,
    remotes: u64,
    leafnodes: u64,
    in_msgs: u64,
    out_msgs: u64,
    in_bytes: u64,
    out_bytes: u64,
    slow_consumers: u64,
    subscriptions: u64,
    config_load_time: String,
    // http_req_stats:
    // gateway:
    // leaf:
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ServerCluster {
    addr: String,
    cluster_port: u16,
    auth_timeout: u64
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NatsClient {
    server: NatsServer
}