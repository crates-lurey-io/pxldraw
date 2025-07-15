use grixy::buf::SliceMutGrid;
use png::Encoder;
use pxldraw::{Color, DrawTarget, PixelBuf, Pos};
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
    // TODO: Make "Color" (Pixel) Copy if the internal representation is Copy.
    let mut buf: [Color<Rgba8888>; WIDTH * HEIGHT] = std::array::from_fn(|_| opaque_black());
    let grid = SliceMutGrid::with_buffer_row_major(&mut buf, WIDTH, HEIGHT).unwrap();
    let mut draw = PixelBuf::new(grid);

    // Draw a box.
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if (x % DENSE == 0) || (y % DENSE == 0) || (x == WIDTH - 1) || (y == HEIGHT - 1) {
                draw.draw_pixel(Pos::new(x, y), opaque_white());
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

    // TODO: Add a into_raw_pixels method to PixelBuf.
    let raw_pixels: &[u8] = unsafe {
        std::slice::from_raw_parts(
            buf.as_ptr().cast::<u8>(),
            buf.len() * std::mem::size_of::<Color<Rgba8888>>(),
        )
    };
    writer.write_image_data(raw_pixels).unwrap();
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
    Color::<Rgba8888>::new(0xFF00_0000)
}

fn opaque_white() -> Color<Rgba8888> {
    Color::<Rgba8888>::new(0xFFFF_FFFF)
}
