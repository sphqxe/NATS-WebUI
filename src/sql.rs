use crate::datatypes::{NatsClient, NatsServer, Publication, SubjectTreeNode};
use rusqlite::{params, Connection, Error, Result};
use serde_json;

pub fn get_db_conn() -> rusqlite::Result<Connection> {
    let path = "./sqlite.db";
    Connection::open(&path)
}

pub fn db_conn() -> Connection {
    get_db_conn().expect("Failed to get connection so SQLite DB.")
}

pub fn db_setup(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS servers (
                id	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
                name	TEXT NOT NULL,
                host	TEXT NOT NULL,
                port	INTEGER NOT NULL,
                monitoring_port	INTEGER NOT NULL,
                subjects	TEXT NOT NULL,
                publications	TEXT NOT NULL
            )",
        params![],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS clients (
            id	INTEGER NOT NULL UNIQUE,
            name    TEXT NOT NULL,
            server_id	INTEGER NOT NULL,
            subjects    TEXT NOT NULL,
            info	INTEGER NOT NULL,
            ping	INTEGER NOT NULL,
            pong	INTEGER NOT NULL,
            ok	INTEGER NOT NULL,
            err	INTEGER NOT NULL,
            publ	INTEGER NOT NULL,
            sub	INTEGER NOT NULL,
            unsub	INTEGER NOT NULL,
            connect	INTEGER NOT NULL,
            msg	INTEGER NOT NULL,
            FOREIGN KEY(server_id) REFERENCES servers(id),
            PRIMARY KEY(id)
        )",
        params![],
    )?;
    Ok(())
}

#[allow(dead_code)]
pub fn db_teardown(conn: &Connection) -> Result<()> {
    conn.execute("DROP TABLE servers", params![])?;
    conn.execute("DROP TABLE clients", params![])?;
    Ok(())
}

pub fn get_servers(conn: &Connection) -> Result<Vec<NatsServer>> {
    let mut ps = conn.prepare("SELECT * FROM servers")?;
    let rs = ps
        .query_map(params![], |row| {
            let sbjs: String = row.get(5)?;
            let pubs: String = row.get(6)?;
            Ok(NatsServer {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                host: row.get(2)?,
                port: row.get(3)?,
                monitoring_port: row.get(4)?,
                varz: None,
                subjects: serde_json::from_str::<Vec<SubjectTreeNode>>(&sbjs)
                    .expect("Failed to parse subject from SQL query as Vec<SubjectTreeNode>"),
                publications: serde_json::from_str::<Vec<Publication>>(&pubs)
                    .expect("Failed to parse publications from SQL query as Vec<Publication>"),
            })
        })?
        .into_iter()
        .filter_map(Result::ok)
        .collect::<Vec<NatsServer>>();
    Ok(rs)
}

pub fn get_clients(conn: &Connection) -> Result<Vec<NatsClient>> {
    let mut ps = conn.prepare("SELECT * FROM clients")?;
    let rs = ps
        .query_map(params![], |row| {
            let sbjs: String = row.get(3)?;
            Ok(NatsClient {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                server_id: row.get(2)?,
                subjects: serde_json::from_str::<Vec<SubjectTreeNode>>(&sbjs)
                    .expect("Failed to parse subject from SQL query as Vec<SubjectTreeNode>"),
                info: row.get(4)?,
                ping: row.get(5)?,
                pong: row.get(6)?,
                ok: row.get(7)?,
                err: row.get(8)?,
                publ: row.get(9)?,
                sub: row.get(10)?,
                unsub: row.get(11)?,
                connect: row.get(12)?,
                msg: row.get(13)?,
            })
        })?
        .into_iter()
        .filter_map(Result::ok)
        .collect::<Vec<NatsClient>>();
    Ok(rs)
}

pub fn insert_client(conn: &Connection, client: NatsClient) -> Result<usize> {
    Ok(conn.execute(
        "INSERT INTO clients (name, server_id, subjects, info, ping, pong, ok, err, publ, sub, unsub, connect, msg)\
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
        params![
            client.name,
            client.server_id,
            serde_json::to_string(&client.subjects).expect("Failed to serialize subjects as JSON."),
            client.info,
            client.ping,
            client.pong,
            client.ok,
            client.err,
            client.publ,
            client.sub,
            client.unsub,
            client.connect,
            client.msg
        ]
    )?)
}

pub fn insert_server(conn: &Connection, server: NatsServer) -> Result<usize> {
    Ok(conn.execute(
        "INSERT INTO servers (name, host, port, monitoring_port, subjects, publications) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            server.name,
            server.host,
            server.port,
            server.monitoring_port,
            serde_json::to_string(&server.subjects).expect("Failed to serialize server subjects as JSON"),
            serde_json::to_string(&server.publications).expect("Failed to serialize server publications as JSON")
        ]
    )?)
}

pub fn update_client(conn: &Connection, client: NatsClient) -> Result<usize> {
    Ok(conn.execute(
        "UPDATE clients SET \
        name = ?1,\
        server_id = ?2,\
        subjects = ?3,\
        info = ?4,\
        ping = ?5,\
        pong = ?6,\
        ok = ?7,\
        err = ?8,\
        publ = ?9,\
        sub = ?10,\
        unsub = ?11,\
        connect = ?12,\
        msg = ?13 WHERE id = ?14",
        params![
            client.name,
            client.server_id,
            serde_json::to_string(&client.subjects).expect("Failed to serialize subjects as JSON."),
            client.info,
            client.ping,
            client.pong,
            client.ok,
            client.err,
            client.publ,
            client.sub,
            client.unsub,
            client.connect,
            client.msg,
            client
                .id
                .expect("Cannot update client without providing an id.")
        ],
    )?)
}

pub fn update_server(conn: &Connection, server: NatsServer) -> Result<usize> {
    Ok(conn.execute(
        "UPDATE servers SET \
        name = ?1,\
        host = ?2,\
        port = ?3,\
        monitoring_port = ?4,\
        subjects = ?5,\
        publications = ?6 WHERE id = ?7",
        params![
            server.name,
            server.host,
            server.port,
            server.monitoring_port,
            serde_json::to_string(&server.subjects)
                .expect("Failed to serialize server subjects as JSON"),
            serde_json::to_string(&server.publications)
                .expect("Failed to serialize server publications as JSON"),
            server
                .id
                .expect("Cannot update server without providing an id.")
        ],
    )?)
}

pub fn delete_client(conn: &Connection, client_id: i64) -> Result<usize> {
    Ok(conn.execute("DELETE FROM clients WHERE id = ?1", params![client_id])?)
}

pub fn delete_server(conn: &Connection, server_id: i64) -> Result<usize> {
    Ok(conn.execute("DELETE FROM servers WHERE id = ?1", params![server_id])?)
}

pub fn get_connection_triple(
    conn: &Connection,
    client_id: i64,
) -> Result<(String, u16, Vec<SubjectTreeNode>)> {
    let mut ps = conn.prepare("SELECT servers.host, servers.port, clients.subjects FROM clients INNER JOIN servers ON clients.server_id=servers.id WHERE clients.id = ?1")?;
    let rs = ps
        .query_map(params![client_id], |row| {
            let sbjs: String = row.get(2)?;
            Ok((
                row.get::<usize, String>(0)?,
                row.get::<usize, u16>(1)?,
                serde_json::from_str::<Vec<SubjectTreeNode>>(&sbjs)
                    .expect("Failed to parse subject from SQL query as Vec<SubjectTreeNode>"),
            ))
        })?
        .into_iter()
        .filter_map(Result::ok)
        .next()
        .ok_or(Error::QueryReturnedNoRows);
    rs
}

#[cfg(test)]
mod test {
    // NOTE: These tests should not be run in parallel
    // Use cargo test sql -- --test-threads=1
    use super::*;

    #[test]
    fn test_get_db_conn() {
        get_db_conn().unwrap();
    }

    #[test]
    fn test_db_setup_and_teardown() {
        let conn = get_db_conn().unwrap();
        db_setup(&conn);
        get_clients(&conn).expect("Could not get clients");
        get_servers(&conn).expect("Could not get servers");
        db_teardown(&conn);
        get_clients(&conn).expect_err("Could not drop clients table.");
        get_servers(&conn).expect_err("Could not drop servers table.");
    }

    fn create_test_server() -> NatsServer {
        NatsServer {
            id: None,
            name: String::from("test_server"),
            host: String::from("test_host.com"),
            port: 4222,
            monitoring_port: 8222,
            varz: None,
            subjects: vec![],
            publications: vec![],
        }
    }

    #[test]
    fn test_servers_crud() {
        let conn = get_db_conn().unwrap();
        db_teardown(&conn);
        db_setup(&conn);

        // Insert a server and check if it can be read
        let test_server = create_test_server();
        insert_server(&conn, test_server);
        let servers = get_servers(&conn).unwrap();
        let mut test_server = create_test_server();
        test_server.id = Some(1);
        assert_eq!(servers, vec![test_server]);

        // Update the server
        let mut test_server = create_test_server();
        test_server.id = Some(1);
        test_server.name = String::from("Updated_name");
        update_server(&conn, test_server.clone());
        let servers = get_servers(&conn).unwrap();
        assert_eq!(servers, vec![test_server]);

        // Delete the record and check that the table is empty
        assert_eq!(delete_server(&conn, 1), Ok(1));
        assert_eq!(get_servers(&conn).unwrap().iter().count(), 0);

        db_teardown(&conn);
    }

    fn create_test_client() -> NatsClient {
        NatsClient {
            id: None,
            name: "test_client".to_string(),
            server_id: 1,
            subjects: vec![],
            info: false,
            ping: true,
            pong: false,
            ok: true,
            err: false,
            publ: true,
            sub: false,
            unsub: true,
            connect: false,
            msg: true,
        }
    }

    #[test]
    fn test_clients_crud() {
        let conn = get_db_conn().unwrap();
        db_teardown(&conn);
        db_setup(&conn);

        // Insert a client and check if it can be read
        let test_client = create_test_client();
        insert_client(&conn, test_client);
        let clients = get_clients(&conn).unwrap();
        let mut test_client = create_test_client();
        test_client.id = Some(1);
        assert_eq!(clients, vec![test_client]);

        // Update the client
        let mut test_client = create_test_client();
        test_client.id = Some(1);
        test_client.name = String::from("Updated_name");
        update_client(&conn, test_client.clone());
        let clients = get_clients(&conn).unwrap();
        assert_eq!(clients, vec![test_client]);

        // Delete the record and check that the table is empty
        assert_eq!(delete_client(&conn, 1), Ok(1));
        assert_eq!(get_clients(&conn).unwrap().iter().count(), 0);

        db_teardown(&conn);
    }
}
