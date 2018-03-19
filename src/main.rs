extern crate gtk;

use std::env;
mod input;
mod logging;

use gtk::prelude::*;
use gtk::{Window, WindowPosition, WindowType, TextView, TextBuffer, Menu, MenuBar, MenuItem};

//Cargo puts the build version into an environment variable at build time
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const LOG_LEVEL_ENV_VAR_NAME: &'static str = "SIMPLE_LOG_LEVEL";
const TITLE: &'static str = "simple";

fn main() {
    //Parse the input to get options
    let args: Vec<String> = env::args().collect();
    let options = input::parse_input_to_options(&args);

    if options.enable_debug_logging {
        enable_debug_logging();
    }

    if options.show_help {
        print_usage();
        return;
    }
    
    if options.show_version {
        print_version();
        return;
    }

    establish_gui();
    gtk::main();
}

fn print_usage() {
    logging::infoln("");
    logging::infoln("Usage:");
    logging::infoln("simple [options]");
    logging::infoln("");
    logging::infoln("Options:");
    logging::infoln("   -h|--help       Show usage documentation");
    logging::infoln("   -v|--verbose    Enable debug logging");
    logging::infoln("   -V|--version    Show version");
    logging::infoln("");
}

fn print_version() {
    logging::infoln(format!("Version: {}",VERSION).as_str());
}

fn enable_debug_logging() {
    env::set_var(LOG_LEVEL_ENV_VAR_NAME, "DEBUG");
    logging::debugln("Debug logging enabled");
}

fn establish_gui() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
    }

    let window = Window::new(WindowType::Toplevel);
    window.set_title(TITLE);
    window.set_default_size(800, 400);
    window.set_position(WindowPosition::Center);

    //Vertical box that will be our main container for everything
    let main_box = gtk::Box::new(gtk::Orientation::Vertical, 1);

    //Create the menu
    let file_menu = Menu::new();
    let menu_bar = MenuBar::new();
    let file_top_level = MenuItem::new_with_label("File");
    let open_file_item = MenuItem::new_with_label("Open...");
    let save_file_item = MenuItem::new_with_label("Save");
    file_menu.append(&open_file_item);
    file_menu.append(&save_file_item);
    file_top_level.set_submenu(Some(&file_menu));
    menu_bar.append(&file_top_level);

    //Create the main text editor view and the buffer behind it
    let main_text_buffer = TextBuffer::new(None);
    let main_text_view = TextView::new_with_buffer(&main_text_buffer);

    //Put the wdigets into the main box container
    main_box.pack_start(&menu_bar, false, false, 0);
    main_box.pack_start(&main_text_view, true, true, 0);

    //Add the main box to the top level window
    window.add(&main_box);
    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
}