use conrod_core::{self, widget, widget_ids, Labelable, Positionable, Sizeable, Widget};
use conrod_experiments_rs::program;
use glium;
use std;

const WIDTH: u32 = 620;
const HEIGHT: u32 = 480;
const FONT_PATH: &str = "data/fonts/NotoSans/NotoSans-Regular.ttf";

fn main() {
    // Init the program
    let mut prog = program::Program::new(
        "Conrod counter",
        WIDTH,
        HEIGHT,
        std::time::Duration::from_millis(16),
    );

    // Add a `Font` to the `Ui`'s `font::Map` from file.
    prog.ui.fonts.insert_from_file(FONT_PATH).unwrap();

    // The hash map containing our images (none here).
    let image_map = conrod_core::image::Map::<glium::texture::Texture2d>::new();

    // The counter
    let mut count = 0;

    // Create our widgets.
    widget_ids!(struct Ids { canvas, counter });
    let ids = Ids::new(prog.ui.widget_id_generator());

    let mut my_widgets = |ui: &mut conrod_core::UiCell| {
        // Create a background canvas upon which we'll place the button.
        widget::Canvas::new().pad(40.0).set(ids.canvas, ui);

        // Draw the button and increment `count` if pressed.
        for _click in widget::Button::new()
            .middle_of(ids.canvas)
            .w_h(80.0, 80.0)
            .label(&count.to_string())
            .set(ids.counter, ui)
        {
            count += 1;
        }
    };

    // Run forever our program.
    prog.run(&image_map, &mut my_widgets);
}
