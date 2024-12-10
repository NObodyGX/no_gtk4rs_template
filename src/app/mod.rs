mod imp;

use adw::prelude::*;
use gtk::glib::clone;
use gtk::{gio, glib};

use crate::main_window::MainWindow;
use crate::perferences::MainPreferences;

glib::wrapper! {
    pub struct NopNameApplication(ObjectSubclass<imp::NopNameApplication>)
        @extends adw::Application, gio::Application, gtk::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl NopNameApplication {
    pub fn new(application_id: &str, flags: &gio::ApplicationFlags) -> Self {
        glib::Object::builder()
            .property("application-id", application_id)
            .property("flags", flags)
            .build()
    }

    fn create_window(&self) -> MainWindow {
        let window = MainWindow::new(&self);
        window.present();
        window
    }

    fn setup_gactions(&self) {
        let preferences_action = gio::SimpleAction::new("preferences", None);
        preferences_action.connect_activate(glib::clone!(
            #[weak(rename_to = app)]
            self,
            move |_action, _parameter| {
                app.show_prefrerences();
            }
        ));
        self.add_action(&preferences_action);

        let quit_action = gio::SimpleAction::new("quit", None);
        quit_action.connect_activate(glib::clone!(
            #[weak(rename_to = app)]
            self,
            move |_action, _parameter| {
                app.quit();
            }
        ));
        self.add_action(&quit_action);

        let about_action = gio::SimpleAction::new("about", None);
        about_action.connect_activate(glib::clone!(
            #[weak(rename_to = app)]
            self,
            move |_action, _parameter| {
                app.show_about();
            }
        ));
        self.add_action(&about_action);
    }

    fn show_prefrerences(&self) {
        let window = self.active_window().unwrap();
        let preferences = MainPreferences::new();
        preferences.set_modal(true);
        preferences.set_transient_for(Some(&window));
        preferences.connect_font_changed(clone!(
            #[weak]
            window,
            move |_| {
                window
                    .activate_action("win.refresh_text_view_font", None)
                    .unwrap();
            },
        ));
        preferences.present();
    }

    fn show_about(&self) {
        let version = env!("CARGO_PKG_VERSION");
        let name = env!("CARGO_PKG_NAME");
        let app_name = "NopName";
        let url = env!("CARGO_PKG_HOMEPAGE");
        let author = env!("CARGO_PKG_AUTHORS");
        let authors = if !author.contains(":") {
            vec![author]
        } else {
            author.split(":").collect()
        };
        let main_author = if !author.contains(":") {
            author
        } else {
            author.split_once(":").unwrap().0
        };

        let window = self.active_window().unwrap();
        let dialog = adw::AboutDialog::builder()
            .application_icon(name)
            .application_name(app_name)
            .version(version)
            .developer_name(main_author)
            .developers(authors)
            .copyright(format!("© 2024 {}", main_author))
            .website(url)
            .issue_url(format!("{url}/issues"))
            .license_type(gtk::License::MitX11)
            .build();
        dialog.present(Some(&window));
    }
}

impl Default for NopNameApplication {
    fn default() -> Self {
        let name = env!("CARGO_PKG_NAME");
        glib::Object::builder()
            .property("application-id", name)
            .build()
    }
}
