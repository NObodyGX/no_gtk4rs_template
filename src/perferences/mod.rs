mod config;
mod imp;

use gtk::glib::clone;
use gtk::{glib, prelude::*, subclass::prelude::*, *};

use crate::perferences::config::Config;

glib::wrapper! {
    pub struct MainPreferences(ObjectSubclass<imp::MainPreferences>)
        @extends Widget, Window, adw::Window,
        @implements Accessible, Buildable, ConstraintTarget, Native, Root, ShortcutManager;
}

impl MainPreferences {
    pub fn new() -> Self {
        glib::Object::new()
    }

    fn setup_config(&self) {
        let iconfig = Config::new();
        let mut config = self.imp().config.borrow_mut();
        config.clone_from(&iconfig);
    }
    fn bind_settings(&self) {
        // 注意: schema 里不能使用 _ 而是需要使用 - 才符合格式
        let imp = self.imp();
        let fdesc = imp.config.borrow().custom_font.clone();
        imp.font
            .set_font_desc(&gtk::pango::FontDescription::from_string(fdesc.as_str()));
        imp.font.connect_font_desc_notify(clone!(
            #[weak]
            imp,
            move |_| {
                let font_string = imp.font.font_desc().unwrap().to_string();

                let mut config = imp.config.borrow_mut();
                config.custom_font = font_string;
                config.save();

                imp.obj().emit_by_name::<()>("font-changed", &[]);
            },
        ));

        let use_custom_font = imp.use_custom_font.get();
        use_custom_font.connect_active_notify(clone!(
            #[weak]
            imp,
            move |switch| {
                let mut config = imp.config.borrow_mut();
                config.use_custom_font = switch.is_active();
                config.save();
            },
        ));
    }

    pub fn connect_font_changed<F: Fn(&Self) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.connect_local("font-changed", true, move |values| {
            let obj = values[0].get().unwrap();
            f(obj);
            None
        })
    }
}

impl Default for MainPreferences {
    fn default() -> Self {
        Self::new()
    }
}
