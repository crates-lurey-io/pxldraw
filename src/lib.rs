//! 2D software pixel renderer.

#![no_std]

pub use grixy::core::Pos;
use grixy::grid::GridWrite;
pub use pxlfmt::core::Pixel as Color;
use pxlfmt::{core::Format, prelude::Rgba8888};

/// Defines a target for drawing pixels on a 2D surface.
pub trait DrawTarget<F: Format = Rgba8888> {
    /// Draws a single pixel at the specified coordinates with the given color.
    fn draw_pixel(&mut self, pos: Pos, color: Color<F>);
}

/// Acts as a drawing target for pixel buffers.
pub struct PixelBuf<T: GridWrite<Element = Color<F>>, F: Format = Rgba8888> {
    inner: T,
}

impl<F: Format, T: GridWrite<Element = Color<F>>> PixelBuf<T, F> {
    /// Creates a new `PixelBuf` from the given grid.
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<F: Format, T: GridWrite<Element = Color<F>>> DrawTarget<F> for PixelBuf<T, F> {
    fn draw_pixel(&mut self, pos: Pos, color: Color<F>) {
        let _ = self.inner.set(pos, color);
    }
}

#[cfg(test)]
mod tests {
    use grixy::{buf::ArrayGrid, core::RowMajor};

    use super::*;

    #[test]
    fn test_draw_pixel() {
        let grid = ArrayGrid::<Color<Rgba8888>, 1, RowMajor>::new_filled(
            1,
            1,
            Color::<Rgba8888>::new(0xFF00_0000),
        );
        let mut pixel_buf = PixelBuf::new(grid);
        pixel_buf.draw_pixel(Pos::new(0, 0), Color::<Rgba8888>::new(0xFFFF_FFFF));

        assert_eq!(
            pixel_buf.inner.get(Pos::new(0, 0)),
            Some(&Color::<Rgba8888>::new(0xFFFF_FFFF))
        );
    }
}
