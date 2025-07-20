use grixy::core::Rect;
use png::Encoder;
use pxldraw::{
    buffer::Framebuffer,
    core::{Color, Pos},
    target::DrawTarget,
};
use pxlfmt::prelude::Rgba8888;

/// Draws a box and outputs it as a PNG `grid.png` in a temp directory and opens it.
///
/// ```sh
/// cargo run --example draw-grid-png
/// ```
fn main() {
    const WIDTH: usize = 100;
    const HEIGHT: usize = 100;
    const DENSE: usize = 10;

    // Create a pixel buffer.
    let mut buffer = Framebuffer::new(WIDTH, HEIGHT);
    buffer
        .fill_rect(Rect::from_ltwh(0, 0, WIDTH, HEIGHT), opaque_black())
        .unwrap();

    // Draw a box.
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if (x % DENSE == 0) || (y % DENSE == 0) || (x == WIDTH - 1) || (y == HEIGHT - 1) {
                buffer.draw_pixel(Pos::new(x, y), opaque_white()).unwrap();
            }
        }
    }

    // Get a temp directory for the output.
    let temp_dir = std::env::temp_dir();

    // Encode the pixel buffer to PNG.
    #[allow(clippy::cast_possible_truncation)]
    let mut encoder = Encoder::new(
        std::fs::File::create(temp_dir.join("grid.png")).unwrap(),
        WIDTH as u32,
        HEIGHT as u32,
    );
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer
        .write_image_data(buffer.as_inner().as_bytes())
        .unwrap();
    println!(
        "Box drawn and saved to: {}",
        temp_dir.join("grid.png").display()
    );

    // Open the PNG file using the default image viewer.
    if let Err(e) = open::that(temp_dir.join("grid.png")) {
        eprintln!("Failed to open the image: {e}");
        std::process::exit(1);
    }
}

fn opaque_black() -> Color<Rgba8888> {
    Color::<Rgba8888>::with_rgba(0x00, 0x00, 0x00, 0xFF)
}

fn opaque_white() -> Color<Rgba8888> {
    Color::<Rgba8888>::with_rgba(0xFF, 0xFF, 0xFF, 0xFF)
}
