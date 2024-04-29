use gtk::{prelude::*, ApplicationWindow};
use gtk::{Application, Button};
use sysinfo::System;

pub struct Ui;

impl Ui {
    pub fn build(app: &Application, system: &System) {
        let button = Ui::create_button(system);
        // Create a window and set the title
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Resource Monitor")
            .child(&button)
            .default_height(1920)
            .default_width(1080)
            .build();

        // Present window
        window.present();
    }

    fn create_button(system: &System) -> Button {
        // Create a button with label and margins
        let button = Button::builder()
            .label("Get processes")
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .build();

        let total_processes = system.processes().len();

        // Connect to "clicked" signal of `button`
        button.connect_clicked(move |button| {
            button.set_label(&format!("Total proccesses: {}", total_processes.to_owned()));
        });

        return button;
    }
}
