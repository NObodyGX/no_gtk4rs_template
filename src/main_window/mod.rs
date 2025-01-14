mod imp;

use glib::Object;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};

use crate::app::NopNameApplication;

glib::wrapper! {
    pub struct MainWindow(ObjectSubclass<imp::MainWindow>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl MainWindow {
    pub fn new(app: &NopNameApplication) -> Self {
        Object::builder().property("application", app).build()
    }

    fn setup_callbacks(&self) {
        self.imp().hello_button.connect_clicked(glib::clone!(
            #[weak(rename_to = window)]
            self,
            move |_parameter| {
                window.hello_callback();
            }
        ));

        self.imp().goodbye_button.connect_clicked(glib::clone!(
            #[weak(rename_to = window)]
            self,
            move |_parameter| {
                window.goodbye_callback();
            }
        ));
    }

    fn hello_callback(&self) {
        let hello_world_label = &self.imp().hello_world_label;
        let buffer = "Hello!";
        hello_world_label.set_text(&buffer);
    }

    fn goodbye_callback(&self) {
        let hello_world_label = &self.imp().hello_world_label;
        let buffer = "Goodbye!";
        hello_world_label.set_text(&buffer);
    }
}
