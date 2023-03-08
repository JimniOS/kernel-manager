use std::fs::read_to_string;
use relm4::prelude::*;
pub mod ui;
use ui::{builder::lib::kernel::Kernel, main::GeneralApp};

pub const APP_ID: &str = "kernel_manager";
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const APP_DEBUG: bool = cfg!(debug_assertions);

fn parse_kernel_info() -> Vec<Kernel> {
    let file_content =
        read_to_string("./kernel.list").expect("Unable to get kernel list informations");

    let tokens = file_content.split("\n\n");
    let mut kernel_list: Vec<Kernel> = vec![];
    tokens.for_each(|token| {
        let sub_token = token.split('\n').collect::<Vec<&str>>();
        let version_name = sub_token[0].replace(['[', ']'], "");

        let url = *sub_token.last().unwrap();
        let download_url = url
            .split('=')
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .replace('\"', "");
        let new_kernel_obj = Kernel {
            url: Some(download_url),
            version: format!("Linux {}", version_name),
            path: None,
        };
        kernel_list.push(new_kernel_obj);
    });
    kernel_list
}
fn main() {
    adw::init().expect("Failed to initialise LibAdwaita");

    // set app title
    gtk::glib::set_application_name("Kernel Manager");
    gtk::glib::set_program_name(Some("Kernel manager"));

    let app = RelmApp::new(APP_ID);

    app.run::<GeneralApp>(parse_kernel_info());
}
