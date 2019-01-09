use conrod_core::{self, color, widget, widget_ids, Colorable, Positionable, Widget};
use conrod_experiments_rs::program;
use glium;
use std;

const WIDTH: u32 = 620;
const HEIGHT: u32 = 480;
const FONT_PATH: &str = "data/fonts/NotoSans/NotoSans-Regular.ttf";

fn main() {
    // Init the program
    let mut prog = program::Program::new(
        "Conrod Hello World",
        WIDTH,
        HEIGHT,
        std::time::Duration::from_millis(16),
    );

    // Add a `Font` to the `Ui`'s `font::Map` from file.
    prog.ui.fonts.insert_from_file(FONT_PATH).unwrap();

    // The hash map containing our images (none here).
    let image_map = conrod_core::image::Map::<glium::texture::Texture2d>::new();

    // Create our widgets.
    widget_ids!(struct Ids { text });
    let ids = Ids::new(prog.ui.widget_id_generator());

    let mut my_widgets = |ui: &mut conrod_core::UiCell| {
        widget::Text::new("Hello World!")
            .middle_of(ui.window)
            .color(color::WHITE)
            .font_size(32)
            .set(ids.text, ui);
    };

    // Run forever our program.
    prog.run(&image_map, &mut my_widgets);
}
