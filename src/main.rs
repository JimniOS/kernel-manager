use relm4::prelude::*;

pub mod ui;
use ui::main::*;

pub const APP_ID:&str = "kernel_manager";
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const APP_DEBUG:bool= cfg!(debug_assertions);


fn main(){
    adw::init().expect("Failed to initialise LibAdwaita");

    // set app title
    gtk::glib::set_application_name("Kernel Manager");
    gtk::glib::set_program_name(Some("Kernel manager"));

    let app = RelmApp::new(APP_ID);
    app.run::<GeneralApp>(());
}