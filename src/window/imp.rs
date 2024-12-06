use glib::subclass::InitializingObject;
use gtk::subclass::prelude::*;
use gtk::{glib, Button, CompositeTemplate, Label};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/github/nobodygx/nopname/ui/main_window.ui")]
pub struct Window {
    #[template_child]
    pub hello_button: TemplateChild<Button>,
    #[template_child]
    pub goodbye_button: TemplateChild<Button>,
    #[template_child]
    pub hello_world_label: TemplateChild<Label>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "nopname";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();
        obj.setup_callbacks();
    }
}

impl WidgetImpl for Window {}

impl WindowImpl for Window {}

impl ApplicationWindowImpl for Window {}
