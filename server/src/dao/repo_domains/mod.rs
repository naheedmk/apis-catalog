extern crate failure;
extern crate rusqlite;
extern crate time;
extern crate uuid;

use uuid::Uuid;

use rusqlite::NO_PARAMS;
use rusqlite::{params, Connection, Result};

//use rustbreak::{FileDatabase, deser::Ron};
use log::debug;

pub struct DomainItem {
    pub name: std::string::String,
    pub id: Uuid,
    pub description: String,
    pub owner: String,
}

pub fn list_all_domains(config: &super::super::settings::Database) -> Result<Vec<DomainItem>> {
    let mut db_path = String::from(&config.rusqlite_path);
    db_path.push_str("/apis-catalog-all.db");
    {
        debug!("Reading all domains from Domain_Database [{:?}]", db_path);
    }

    let conn = Connection::open(db_path)?;

    let mut stmt = conn.prepare("SELECT id, name, description, owner FROM domains")?;
    let mut rows = stmt.query(NO_PARAMS)?;

    let mut tuples = Vec::new();
    while let Some(row) = rows.next()? {
        let id = row.get("id")?;
        let name = row.get("name")?;
        let descripton = row.get("description")?;
        let owner = row.get("owner")?;
        let domain = DomainItem {
            id: id,
            name: name,
            description: descripton,
            owner: owner,
        };

        tuples.push(domain);
    }

    Ok(tuples)
}

pub fn add_domain(
    config: &super::super::settings::Database,
    name: &str,
    description: &str,
    owner: &str,
) -> Result<Uuid> {
    let mut db_path = String::from(&config.rusqlite_path);
    db_path.push_str("/apis-catalog-all.db");
    {
        debug!(
            "Creating domain [{}] into Domain_Database [{:?}]",
            name, db_path
        );
    }

    let conn = Connection::open(db_path)?;

    let id = Uuid::new_v4();
    conn.execute(
        "INSERT INTO domains (id, name, description, owner) VALUES (?1, ?2, ?3, ?4)",
        params![id, name, description, owner],
    )?;

    conn.close().unwrap();
    Ok(id)
}

pub fn get_domain(config: &super::super::settings::Database, id: Uuid) -> Result<DomainItem> {
    let mut db_path = String::from(&config.rusqlite_path);
    db_path.push_str("/apis-catalog-all.db");
    {
        debug!("Get domain [{}] into Domain_Database [{:?}]", id, db_path);
    }

    let conn = Connection::open(db_path)?;

    let mut stmt = conn.prepare("SELECT id, name, description FROM domains WHERE id = ?1")?;
    let row = stmt.query_row(params![id], |row| {
        Ok(DomainItem {
            name: row.get(1)?,
            id: row.get(0)?,
            description: row.get(2)?,
            owner: row.get(3)?,
        })
    })?;

    Ok(row)
}

pub fn delete_domain(config: &super::super::settings::Database, id: Uuid) -> Result<()> {
    let mut db_path = String::from(&config.rusqlite_path);
    db_path.push_str("/apis-catalog-all.db");
    {
        debug!(
            "Delete domain [{}] into Domain_Database [{:?}]",
            id, db_path
        );
    }

    let conn = Connection::open(db_path)?;

    let mut stmt = conn.prepare("DELETE FROM domains where id = ?1")?;
    stmt.execute(params![id])?;

    Ok(())
}
