use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button};

const APP_ID: &str = "com.teacherschedule.app";

fn main() -> glib::ExitCode {
    // crea una nuova applicazione
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    // crea un pulsante
    let button = Button::builder()
        .label("Cliccami!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    // crea il comportamento del pulsante
    button.connect_clicked(|button| {
        button.set_label("Good Boy!");
    });
    // crea la finestra base
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Creatore tabelle orari docenti")
        .child(&button)
        .build();
    // mostra la finestra
    window.present();
}
