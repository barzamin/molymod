use std::{fmt, path::Path};

use nalgebra::{self as na, Matrix3, Vector3};
use rusqlite::{Connection, OpenFlags};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("backing database error")]
    Db(#[from] rusqlite::Error),

    #[error("global_cell table malformed")]
    GlobalCellMalformed,
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

    pub fn get_ver(&self) -> Result<DMSVer, Error> {
        Ok(self
            .db
            .query_row("SELECT major, minor FROM dms_version", [], |row| {
                Ok(DMSVer::new(row.get("major")?, row.get("minor")?))
            })?)
    }

    pub fn global_cell(&self) -> Result<Matrix3<f64>, Error> {
        let mut stmt = self.db.prepare("SELECT id, x, y, z FROM global_cell")?;
        let mut rows = stmt.query([])?;

        let mut globcell: Matrix3<f64> = Matrix3::zeros();
        for i in 0..3 {
            let row = rows.next()?.ok_or_else(|| Error::GlobalCellMalformed)?;
            let id: i32 = row.get(0)?;
            if (id as usize) - 1 != i {
                return Err(Error::GlobalCellMalformed);
            }
            let x: f64 = row.get(1)?;
            let y: f64 = row.get(2)?;
            let z: f64 = row.get(3)?;

            globcell[(i, 0)] = x;
            globcell[(i, 1)] = y;
            globcell[(i, 2)] = z;
        }

        Ok(globcell)
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

#[derive(Debug)]
pub struct DMSVer {
    major: i32,
    minor: i32,
}

impl fmt::Display for DMSVer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

impl DMSVer {
    pub fn new(major: i32, minor: i32) -> Self {
        Self { major, minor }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_2f4k_smoke() {
        let dms = DMSFile::open("tests/2f4k.dms").unwrap();
    }
}
