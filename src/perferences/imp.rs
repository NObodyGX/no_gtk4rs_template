use adw::subclass::prelude::AdwWindowImpl;
use adw::subclass::prelude::PreferencesWindowImpl;
use glib::subclass::Signal;
use gtk::{glib, subclass::prelude::*, CompositeTemplate, *};
use std::{cell::RefCell, sync::OnceLock};

use super::config::Config;

#[derive(Debug, Default, CompositeTemplate)]
#[template(resource = "/com/github/noa-name/nop-name/ui/preferences.ui")]
pub struct MainPreferences {
    #[template_child]
    pub use_custom_font: TemplateChild<Switch>,
    #[template_child]
    pub font: TemplateChild<FontDialogButton>,

    pub config: RefCell<Config>,
}

#[glib::object_subclass]
impl ObjectSubclass for MainPreferences {
    const NAME: &'static str = "MainPreferences";
    type Type = super::MainPreferences;
    type ParentType = adw::PreferencesWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for MainPreferences {
    fn constructed(&self) {
        let obj = self.obj();
        self.parent_constructed();

        obj.setup_config();
        obj.bind_settings();
    }

    fn signals() -> &'static [Signal] {
        static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
        SIGNALS.get_or_init(|| {
            vec![
                Signal::builder("font-changed").build(),
                Signal::builder("theme-changed").build(),
            ]
        })
    }
}
impl WidgetImpl for MainPreferences {}
impl WindowImpl for MainPreferences {}
impl AdwWindowImpl for MainPreferences {}
impl PreferencesWindowImpl for MainPreferences {}
