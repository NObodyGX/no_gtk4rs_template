use glib::subclass::InitializingObject;
use gtk::subclass::prelude::*;
use gtk::{glib, Button, CompositeTemplate, Label, MenuButton};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/github/noa-name/nop-name/ui/main_window.ui")]
pub struct MainWindow {
    #[template_child]
    pub main_menu_button: TemplateChild<MenuButton>,
    #[template_child]
    pub hello_button: TemplateChild<Button>,
    #[template_child]
    pub goodbye_button: TemplateChild<Button>,
    #[template_child]
    pub hello_world_label: TemplateChild<Label>,
}

#[glib::object_subclass]
impl ObjectSubclass for MainWindow {
    const NAME: &'static str = "MainWindow";
    type Type = super::MainWindow;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();

        klass.install_action("win.hello-to-world", None, |window, _, _| {
            window.hello_callback();
        });

        klass.install_action("win.refresh_text_view_font", None, |window, _, _| {
            window.hello_callback();
        });
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for MainWindow {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();
        obj.setup_callbacks();
    }
}

impl WidgetImpl for MainWindow {}

impl WindowImpl for MainWindow {}

impl ApplicationWindowImpl for MainWindow {}
