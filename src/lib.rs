mod docente {
    use chrono::{NaiveDate, NaiveTime};

    enum TipoAssunzione {
        Ordinario,
        Emerito,
        Straordinario,
        Invitato,
        Associato,
    }

    enum StatoDocente {
        Attivo,
        Disattivato,
        Malattia,
        Ferie,
        Permesso,
    }

    struct Docente {
        id: u32,
        nome: String,
        cognome: String,
        data_nascita: NaiveDate,
        tipo_assunzione: TipoAssunzione,
        ore_settimanali: u32,
        stato: StatoDocente,
        materie: Vec<Materia>,
        classi: Vec<Classe>,
        altri_ruoli: Vec<RuoloAggiuntivo>,
    }

    struct Materia {
        id: u32,
        nome: String,
        codice_identificativo: String,
        crediti: u32,
        valutazione_min: u32,
        valutazione_max: u32,
        tipo_esame: String,
        docente: Box<Docente>,
        inizio_corso: String,
        fine_corso: String,
    }

    struct Classe {
        id: u32,
        grado: u32,
        sezione: u32,
        nome: String,
        numero_alunni: u32,
        docenti: Vec<Docente>,
        materie: Vec<Materia>,
    }

    struct RuoloAggiuntivo {
        id: u32,
        nome: String,
        docente: Box<Docente>,
    }

    struct Orario {
        id: u32,
        docente: Box<Docente>,
        ora_inizio: NaiveTime,
        ora_fine: NaiveTime,
        classe: Box<Classe>,
    }

    struct Istituto {
        id: u32,
        numero_classi: u32,
        classi: Vec<Classe>,
        preside: Docente,
        vicepreside: Docente,
        numerotelefono: String,
    }
}
// nome, cognome, data-nascita, tipo assunzione (ordinario, straordinario, invitato, emerito,
// associato), ore settimanali, stato (attivo, disattivo), materie, classi, ruoli_aggiuntivi
