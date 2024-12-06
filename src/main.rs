mod app;
mod window;

use gtk::gio;
use gtk::glib;
use gtk::prelude::*;
use rust_embed::Embed;
use window::Window;

#[derive(Embed)]
#[folder = "data_store"]
struct Asset;

fn load_resource() {
    let fname = "nopname.gresource";
    let resource = if Asset::get(fname).is_some() {
        let emfile = Asset::get(fname).unwrap();
        let emdata = emfile.data.into_owned();
        let data = glib::Bytes::from_owned(emdata);
        gio::Resource::from_data(&data).unwrap()
    } else {
        // gio::resources_register_include!("nopname.gresource")
        //     .expect("Failed to register resources.");
        panic!("no nopname.gresource found");
    };
    gio::resources_register(&resource);
}

fn main() {
    load_resource();

    // Create a new application
    let app = adw::Application::builder()
        .application_id("org.nobodygx.nopname")
        .build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &adw::Application) {
    // Create a new custom window and show it
    let window = Window::new(app);
    window.present();
}
