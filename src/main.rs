mod db {
    pub mod connessione;
    pub mod operazioni;
}
use db::connessione::Database;
use db::operazioni::OperazioniPreliminariTabelle;

fn main() -> rusqlite::Result<()> {
    // inizializza la connessione
    let db = Database::new()?;
    // crea le tabelle necessarie al progetto
    db.inizializza_tabelle()?;

    Ok(())
}
