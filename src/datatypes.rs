use rants::{Subject, SubjectBuilder};
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct App {
    pub servers: Vec<NatsServer>,
    pub clients: Vec<NatsClient>,
}

impl App {
    pub fn set_servers(&mut self, servers: Vec<NatsServer>) {
        self.servers = servers;
    }

    pub fn set_clients(&mut self, clients: Vec<NatsClient>) {
        self.clients = clients;
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct NatsServer {
    pub id: Option<i64>,
    pub name: String,
    pub host: String,
    pub port: u16,
    pub monitoring_port: u16,
    pub varz: Option<ServerVarz>,
    pub subjects: Vec<SubjectTreeNode>,
    pub publications: Vec<Publication>,
}

impl NatsServer {
    pub async fn get_varz(
        id: i64,
        host: String,
        port: u16,
        client: &reqwest::Client,
    ) -> reqwest::Result<VarzBroadcastMessage> {
        let varz = client
            .get(&format!("http://{}:{}/varz", host, port))
            .send()
            .await?
            .json::<ServerVarz>()
            .await?;
        // info!("{:?}", varz);
        Ok(VarzBroadcastMessage {
            server_id: id,
            varz: varz,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VarzBroadcastMessage {
    server_id: i64,
    varz: ServerVarz,
}

#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct SubjectTreeNode {
    id: String,
    subject_str: String,
    subjects: Vec<SubjectTreeNode>,
    selected: bool
}

impl SubjectTreeNode {
    fn get_subscriptions(&self, tokens: &mut Vec<String>, subscriptions: &mut Vec<Subject>) {
        tokens.push(self.subject_str.clone());
        let mut builder = SubjectBuilder::new();

        if self.selected {
            for token in tokens.iter() {
                builder = builder.add(token.clone());
            }
            subscriptions.push(builder.build());
        }

        if !self.subjects.is_empty() {
            for s in self.subjects.iter() {
                s.get_subscriptions(tokens, subscriptions);
            }
        }

        tokens.pop();
    }
}

impl Into<Vec<rants::Subject>> for SubjectTreeNode {
    fn into(self) -> Vec<Subject> {
        let (mut tokens, mut subs) = (Vec::new(), Vec::new());
        self.get_subscriptions(&mut tokens, &mut subs);
        subs
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Publication {
    subject: String,
    message: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
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

#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct ServerCluster {
    addr: String,
    cluster_port: u16,
    auth_timeout: u64,
}

#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct NatsClient {
    pub id: Option<i64>,
    pub name: String,
    pub server_id: i64,
    pub subjects: Vec<SubjectTreeNode>,
    pub info: bool,
    pub ping: bool,
    pub pong: bool,
    pub ok: bool,
    pub err: bool,
    pub publ: bool,
    pub sub: bool,
    pub unsub: bool,
    pub connect: bool,
    pub msg: bool,
}

impl NatsClient {
    pub fn get_subscriptions(&self) -> Vec<rants::Subject> {
        let (mut tokens, mut subs) = (Vec::new(), Vec::new());
        for s in self.subjects.iter() {
            s.get_subscriptions(&mut tokens, &mut subs);
        }
        subs
    }
}
