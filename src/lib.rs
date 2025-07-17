//! 2D software pixel renderer.

#![no_std]

pub use grixy::core::Pos;
use grixy::grid::GridWrite;
pub use pxlfmt::pixel::Pixel as Color;
use pxlfmt::{pixel::Format, prelude::Rgba8888};

/// Defines a target for drawing pixels on a 2D surface.
pub trait DrawTarget<F: Format = Rgba8888> {
    /// Draws a single pixel at the specified coordinates with the given color.
    fn draw_pixel(&mut self, pos: Pos, color: Color<F>);
}

/// A framebuffer.
///
/// Any target that can implements [`GridWrite<Element = Color>`] is a valid framebuffer.
///
/// ## Examples
///
/// ```rust
/// use grixy::buf::ArrayGrid;
/// use pxldraw::{Color, DrawTarget, Framebuffer, Pos};
/// use pxlfmt::prelude::Rgba8888;
///
/// let grid = ArrayGrid::<Color<Rgba8888>, 100, grixy::core::RowMajor>::new_filled(
///     10,
///     10,
///     Color::<Rgba8888>::new(0xFF00_0000),
/// );
///
/// let mut framebuffer = Framebuffer::new(grid);
/// framebuffer.draw_pixel(Pos::new(5, 5), Color::<Rgba8888>::new(0xFFFF_FFFF));
/// ```
pub struct Framebuffer<T, F = Rgba8888>
where
    F: Format,
    T: GridWrite<Element = Color<F>>,
{
    inner: T,
}

impl<F: Format, T: GridWrite<Element = Color<F>>> Framebuffer<T, F> {
    /// Creates a new `Framebuffer` from the given grid.
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<F: Format, T: GridWrite<Element = Color<F>>> DrawTarget<F> for Framebuffer<T, F> {
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
        let mut pixel_buf = Framebuffer::new(grid);
        pixel_buf.draw_pixel(Pos::new(0, 0), Color::<Rgba8888>::new(0xFFFF_FFFF));

        assert_eq!(
            pixel_buf.inner.get(Pos::new(0, 0)),
            Some(&Color::<Rgba8888>::new(0xFFFF_FFFF))
        );
    }
}
