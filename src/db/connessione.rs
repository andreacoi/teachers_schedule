use dirs_next::data_dir;
use std::fs;
use std::path::PathBuf;

// src/db/connessione.rs
use rusqlite::{Connection, Result};

pub struct Database {
    pub connessione: Connection,
}

impl Database {
    pub fn new() -> Result<Self> {
        let percorso_db = get_db_path();
        let connessione = Connection::open(percorso_db)?;
        Ok(Database { connessione })
    }
}

// non si inserisce nell'interfaccia perché si tratta di una funzione cosidetta di utilità.
pub fn get_db_path() -> PathBuf {
    // Ottiene la directory dei dati dell'applicazione
    let mut path = data_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("teachers_schedule");
    path.push("data");
    path.push("my_database.db");
    if let Some(path) = path.parent() {
        fs::create_dir_all(path);
    } else {
        panic!("Impossibile creare la cartella. Controlla i permessi del sistema operativo");
    }
    path
}
