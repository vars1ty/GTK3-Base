// This serves as your root window space.

use gtk::{traits::*, *};

/// Builds the main window widgets.
pub fn build(window: &ApplicationWindow) {
    // Box we're using to draw widgets onto.
    let draw_box = Box::new(Orientation::Horizontal, 0);

    // Add the box.
    window.add(&draw_box);

    // Draws the widgets.
    draw_widgets(&draw_box);
}

/// Draws the widgets.
fn draw_widgets(draw_box: &Box) {
    // Adds a single test label to the center.
    let test_label = Label::new(Some("Hello from GTK3!"));
    draw_box.set_center_widget(Some(&test_label))
}
