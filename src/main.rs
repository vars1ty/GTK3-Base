use gtk::gdk::Screen;
use gtk::traits::*;
use gtk::{prelude::*, *};

// Imports `window`
mod window;

// RGBA Color Values.
const R: f64 = 255.0;
const G: f64 = 255.0;
const B: f64 = 255.0;
const A: f64 = 255.0;

/// Initializes the status bar.
fn activate(application: &Application) {
    // Create a normal GTK window however you like
    let window = ApplicationWindow::new(application);

    window.set_title("GTK3 Template Application");
    window.set_default_size(200, 50);

    // Needed for transparency support.
    window.connect_screen_changed(set_visual);
    window.connect_draw(draw);
    window.set_app_paintable(true);

    // Builds the window.
    window::build(&window);
    window.show_all();
    println!("Window ready!")
}

/// Loads the CSS
#[allow(unused_must_use)]
fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_data(include_bytes!("resources/style.css"));

    // Add the provider to the default screen
    StyleContext::add_provider_for_screen(
        &Screen::default().expect("[ERROR] Could not connect to a display!\n"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

/// Called upon application startup.
fn main() {
    let application = Application::new(None, Default::default());
    application.connect_startup(|_| load_css());
    // Activate the layer shell.
    application.connect_activate(|app| {
        activate(app);
    });

    application.run();
}

/// Applies custom visuals.
fn set_visual(window: &ApplicationWindow, _screen: Option<&gdk::Screen>) {
    if let Some(screen) = window.screen() {
        if let Some(ref visual) = screen.rgba_visual() {
            window.set_visual(Some(visual)); // crucial for transparency
        }
    }
}

/// Draws the window using a custom color and opacity.
fn draw(_: &ApplicationWindow, ctx: &cairo::Context) -> Inhibit {
    ctx.set_source_rgba(R / 255.0, G / 255.0, B / 255.0, A / 255.0);
    ctx.set_operator(cairo::Operator::Screen);
    ctx.paint().expect("[ERROR] Failed painting!\n");
    Inhibit(false)
}
