use rusqlite::{params, Connection, Result};
use crate::datatypes::{NatsServer, SubjectTreeNode, Publication};
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
            )", params![],
    )?;
    Ok(())
}

pub fn db_teardown(conn: &Connection) -> Result<()> {
    conn.execute("DROP TABLE servers", params![])?;
    Ok(())
}

pub fn get_servers(conn: &Connection) -> Result<Vec<NatsServer>> {
    let mut ps = conn.prepare("SELECT * FROM servers")?;
    let rs = ps.query_map(params![], |row| {
        let sbjs: String = row.get(5)?;
        let pubs: String = row.get(6)?;
        Ok(NatsServer {
            id: row.get(0)?,
            name: row.get(1)?,
            host: row.get(2)?,
            port: row.get(3)?,
            monitoring_port: row.get(4)?,
            varz: None,
            // subjects: vec![],
            // publications: vec![]
            subjects: serde_json::from_str::<Vec<SubjectTreeNode>>(&sbjs).expect("Failed to parse subject from SQL query as Vec<SubjectTreeNode>"),
            publications: serde_json::from_str::<Vec<Publication>>(&pubs).expect("Failed to parse publications from SQL query as Vec<Publication>")
        })
    })?.into_iter().filter_map(Result::ok).collect::<Vec<NatsServer>>();
    Ok(rs)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_db_conn() {
        get_db_conn().unwrap();
    }

    #[test]
    fn test_get_servers() {
        let db_conn = get_db_conn().unwrap();
        get_servers(&db_conn).unwrap().iter().for_each(|server| {
            println!("{:?}", server);
        });
    }
}