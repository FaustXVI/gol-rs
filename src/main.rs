extern crate cairo;
extern crate gtk;
extern crate gio;
extern crate glib;

use std::thread;
use kata::gui::build_ui;
use kata::game_of_life::api_types::{Grid, Size};
use kata::game_of_life::run_gol;
use std::prelude::v1::*;
use glib::{Continue, MainContext};

fn main() {
    if gtk::init().is_err() {
        panic!("Failed to initialize GTK.");
    }
    let (ready_tx, ready_rx) = MainContext::sync_channel::<Box<dyn Grid + Send>>(glib::PRIORITY_DEFAULT, 0);
    thread::spawn(move || {
        run_gol(Size { height: 200, width: 200 }, |b| ready_tx.send(b).is_ok());
    });
    let printer = build_ui();
    ready_rx.attach(None, move |grid| {
        printer(grid);
        Continue(true)
    });

    gtk::main();
}

