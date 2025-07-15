//! 2D software pixel renderer.

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
