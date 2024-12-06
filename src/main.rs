mod app;
mod window;

use gtk::gio;
use gtk::prelude::*;
use window::Window;

fn main() {
    // Register and include resources
    gio::resources_register_include!("nopname.gresource").expect("Failed to register resources.");

    // Create a new application
    let app = adw::Application::builder()
        .application_id("org.nobodygx.gtk4rstemplate")
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
