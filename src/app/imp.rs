use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::glib;

#[derive(Debug, Default)]
pub struct NopNameApplication {}

#[glib::object_subclass]
impl ObjectSubclass for NopNameApplication {
    const NAME: &'static str = "NopNameApplication";
    type Type = super::NopNameApplication;
    type ParentType = adw::Application;
}

impl ObjectImpl for NopNameApplication {
    fn constructed(&self) {
        let obj = self.obj();
        self.parent_constructed();

        obj.setup_gactions();
    }
}
impl ApplicationImpl for NopNameApplication {
    fn activate(&self) {
        self.parent_activate();

        let obj = self.obj();
        let app = obj.downcast_ref::<super::NopNameApplication>().unwrap();
        let window = app.create_window();
        window.set_default_size(1280, 720);
        window.set_title(Some("NopName"));
        window.set_icon_name(Some("nop_name"));
        window.present();
    }
}
impl AdwApplicationImpl for NopNameApplication {}
impl GtkApplicationImpl for NopNameApplication {}
