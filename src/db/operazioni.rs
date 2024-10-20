// src/db/operazioni.rs
use crate::db::connessione::Database;
use rusqlite::Result;

pub trait OperazioniPreliminariTabelle {
    fn check_esistenza_database(&self) -> bool;
    fn check_esistenza_tabelle(&self) -> bool;
    fn inizializza_tabelle(&self) -> Result<()>;
}

impl OperazioniPreliminariTabelle for Database {
    fn check_esistenza_database(&self) -> bool {
        true
    }
    fn check_esistenza_tabelle(&self) -> bool {
        true
    }
    fn inizializza_tabelle(&self) -> Result<()> {
        self.connessione.execute(
            "CREATE TABLE pino (id INTEGER PRIMARY KEY, name TEXT NOT NULL, surname TEXT NOT NULL)",
            (),
        )?;
        Ok(())
    }
}
