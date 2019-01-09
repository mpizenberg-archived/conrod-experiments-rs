use conrod_core::{self, color, widget, widget_ids, Colorable, Positionable, Sizeable, Widget};
use conrod_experiments_rs::program;
use glium;
use image;
use std;

const WIDTH: u32 = 620;
const HEIGHT: u32 = 480;

fn main() {
    // Init the program
    let mut prog = program::Program::new(
        "Conrod image example",
        WIDTH,
        HEIGHT,
        std::time::Duration::from_millis(16),
    );

    // Load our image from files, convert it into a texture for widgets.
    let raw_image = load_raw_image("data/rust.png");
    let texture = glium::texture::Texture2d::new(&prog.display.0, raw_image).unwrap();
    let (w, h) = (texture.width(), texture.height());

    // Create a hashmap containing our image data for the widgets.
    let mut image_map = conrod_core::image::Map::new();
    let texture = image_map.insert(texture);

    // Create our widgets.
    widget_ids!(struct Ids { background, texture });
    let ids = Ids::new(prog.ui.widget_id_generator());

    let mut my_widgets = |ui: &mut conrod_core::UiCell| {
        // Draw a light blue background.
        widget::Canvas::new()
            .color(color::LIGHT_BLUE)
            .set(ids.background, ui);
        // Instantiate the `Image` at its full size in the middle of the window.
        widget::Image::new(texture)
            .w_h(w as f64, h as f64)
            .middle()
            .set(ids.texture, ui);
    };

    // Run forever our program.
    prog.run(&image_map, &mut my_widgets);
}

// Function loading an image from a path.
fn load_raw_image(path: &str) -> glium::texture::RawImage2d<u8> {
    let img_path = std::path::Path::new(path);
    let img_rgba = image::open(&img_path).expect("Cannot open image").to_rgba();
    let img_size = img_rgba.dimensions();
    glium::texture::RawImage2d::from_raw_rgba_reversed(&img_rgba.into_raw(), img_size)
}
