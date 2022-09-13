use std::path::Path;

use rusqlite::{Connection, OpenFlags};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("backing database error")]
    Db(#[from] rusqlite::Error),
}

pub struct DMSFile {
    db: Connection,
}

impl DMSFile {
    pub fn open<P>(p: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        Ok(Self {
            db: Connection::open_with_flags(
                p,
                OpenFlags::SQLITE_OPEN_READ_WRITE
                    | OpenFlags::SQLITE_OPEN_CREATE
                    | OpenFlags::SQLITE_OPEN_URI
                    | OpenFlags::SQLITE_OPEN_NO_MUTEX,
            )?,
        })
    }

    pub fn get_ver(&self) -> Result<(i32, i32), Error> {
        Ok(self
            .db
            .query_row("SELECT major, minor FROM dms_version", [], |row| {
                Ok((row.get("major")?, row.get("minor")?))
            })?)
    }

    pub fn provenance(&self) -> Result<Vec<ProvenanceRecord>, Error> {
        Ok(self
            .db
            .prepare(
                "SELECT id, version, timestamp, user, workdir, cmdline, executable FROM provenance",
            )?
            .query_map([], |row| {
                Ok(ProvenanceRecord {
                    id: row.get(0)?,
                    version: row.get(1)?,
                    timestamp: row.get(2)?,
                    user: row.get(3)?,
                    workdir: row.get(4)?,
                    cmdline: row.get(5)?,
                    executable: row.get(6)?,
                })
            })?
            .collect::<Result<Vec<ProvenanceRecord>, rusqlite::Error>>()?)
    }
}

#[derive(Debug)]
pub struct ProvenanceRecord {
    pub id: i32,
    pub version: String,
    pub timestamp: String,
    pub user: String,
    pub workdir: String,
    pub cmdline: String,
    pub executable: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_2f4k_smoke() {
        let dms = DMSFile::open("tests/2f4k.dms").unwrap();
    }
}
