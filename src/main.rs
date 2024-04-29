use std::borrow::Borrow;

use sysinfo::System;

use gtk::prelude::*;
use gtk::{glib, Application};
use ui::ui::Ui;

mod ui;

const APP_ID: &str = "org.gtk_rs.resource-monitor";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    let mut system = System::new_all();
    // We always need to refresh to begin with (and any situation where we want to get the latest
    // information)
    system.refresh_all();

    // Connect to "activate" signal of `app`
    app.connect_activate(move |app| return Ui::build(app, system.borrow()));

    // Run the application
    app.run()
}
