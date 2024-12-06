mod imp;

use std::process::{Command, Stdio};

use crate::app;

use glib::{clone, Object};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};
use adw::Application;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &adw::Application) -> Self {
        Object::builder().property("application", app).build()
    }

    fn setup_callbacks(&self) {
        self.imp().hello_button.connect_clicked(
            clone!(@weak self as window => move |_| {
                window.hello_callback()
            }),
        );

        self.imp()
            .goodbye_button
            .connect_clicked(clone!(@weak self as window => move |_| {
                window.goodbye_callback()
            }));
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
