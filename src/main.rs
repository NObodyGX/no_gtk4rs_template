mod app;
mod main_window;
mod perferences;

use app::NopNameApplication;
use gtk::gio;
use gtk::glib;
use gtk::prelude::*;
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "data_store"]
struct Asset;

fn load_resource() {
    let fname = "nop_name.gresource";
    let resource = if Asset::get(fname).is_some() {
        let emfile = Asset::get(fname).unwrap();
        let emdata = emfile.data.into_owned();
        let data = glib::Bytes::from_owned(emdata);
        gio::Resource::from_data(&data).unwrap()
    } else {
        // gio::resources_register_include!("nop_name.gresource")
        //     .expect("Failed to register resources.");
        panic!("no nop_name.gresource found");
    };
    gio::resources_register(&resource);
}

fn main() {
    load_resource();

    let app = NopNameApplication::new(
        "com.github.noa-name.nop-name",
        &gio::ApplicationFlags::empty(),
    );
    app.connect_startup(|app| {
        setup_shortcuts(app);
    });
    app.run();
}

fn setup_shortcuts(app: &NopNameApplication) {
    app.set_accels_for_action("app.quit", &["<Ctrl>q"]);
    app.set_accels_for_action("win.hello-to-world", &["<Ctrl>h"]);
}
