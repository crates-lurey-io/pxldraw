//! 2D software pixel renderer.

#![no_std]

pub use grixy::core::{Pos, Rect};
use grixy::grid::{GridRead, GridWrite};
pub use pxlfmt::pixel::Pixel as Color;
use pxlfmt::{pixel::Format, prelude::Rgba8888};

/// Defines a target for drawing pixels on a 2D surface.
pub trait DrawTarget<F: Format = Rgba8888> {
    /// Draws a single pixel at the specified coordinates with the given color.
    ///
    /// If the coordinates are out of bounds, the pixel is not drawn.
    fn draw_pixel(&mut self, pos: Pos, color: Color<F>);

    /// Fills a rectangular region with the specified color.
    ///
    /// If the rectangle is out of bounds, it fills only the part that is within bounds.
    ///
    /// ## Implementation
    ///
    /// The default implementation iterates over the rectangle's area and draws each pixel; it may
    /// be overridden for more efficient implementations (such as using something that maps to
    /// less write operations).
    fn fill_rect(&mut self, rect: Rect, color: Color<F>) {
        for y in rect.top()..rect.bottom() {
            for x in rect.left()..rect.right() {
                self.draw_pixel(Pos::new(x, y), color);
            }
        }
    }

    /// Copies a rectangular region from one position to another.
    ///
    /// If the source rectangle is out of bounds, it copies only the part that is within bounds.
    ///
    /// ## Implementation
    ///
    /// The default implementation iterates over the source rectangle's area and draws each pixel at
    /// the destination position; it may be overridden for more efficient implementations (such as
    /// using something that maps to less write operations).
    fn copy_rect(&mut self, from: impl GridRead<Element = Color<F>>, src: Rect, dst: Pos) {
        for y in src.top()..src.bottom() {
            for x in src.left()..src.right() {
                if let Some(color) = from.get(Pos::new(x, y)) {
                    self.draw_pixel(
                        Pos::new(dst.x + (x - src.left()), dst.y + (y - src.top())),
                        *color,
                    );
                }
            }
        }
    }
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
/// let mut framebuffer = Framebuffer::from(grid);
/// framebuffer.draw_pixel(Pos::new(5, 5), Color::<Rgba8888>::new(0xFFFF_FFFF));
/// ```
pub struct Framebuffer<T, F = Rgba8888>
where
    F: Format,
    T: GridWrite<Element = Color<F>>,
{
    inner: T,
}

impl<T, F> From<T> for Framebuffer<T, F>
where
    F: Format,
    T: GridWrite<Element = Color<F>>,
{
    fn from(inner: T) -> Self {
        Self { inner }
    }
}

impl<T, F> DrawTarget<F> for Framebuffer<T, F>
where
    F: Format,
    T: GridWrite<Element = Color<F>>,
{
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
        let mut pixel_buf = Framebuffer::from(grid);
        pixel_buf.draw_pixel(Pos::new(0, 0), Color::<Rgba8888>::new(0xFFFF_FFFF));

        assert_eq!(
            pixel_buf.inner.get(Pos::new(0, 0)),
            Some(&Color::<Rgba8888>::new(0xFFFF_FFFF))
        );
    }

    #[test]
    fn test_fill_rect() {
        let black = Color::<Rgba8888>::new(0xFF00_0000);
        let white = Color::<Rgba8888>::new(0xFFFF_FFFF);

        let grid = ArrayGrid::<Color<Rgba8888>, 25, RowMajor>::new_filled(5, 5, black);
        let mut pixel_buf = Framebuffer::from(grid);
        pixel_buf.fill_rect(
            Rect::from_tlbr(Pos::new(1, 1), Pos::new(4, 4)).unwrap(),
            white,
        );

        #[rustfmt::skip]
        assert_eq!(
            pixel_buf.inner.into_inner().0,
            [
                black, black, black, black, black,
                black, white, white, white, black,
                black, white, white, white, black,
                black, white, white, white, black,
                black, black, black, black, black,
            ]
        );
    }

    #[test]
    fn test_copy_rect() {
        let black = Color::<Rgba8888>::new(0xFF00_0000);
        let white = Color::<Rgba8888>::new(0xFFFF_FFFF);

        // Create a 3x3 checkerboard source buffer.
        let mut src_grid = ArrayGrid::<Color<Rgba8888>, 9, RowMajor>::new_filled(3, 3, black);
        for y in 0..3 {
            for x in 0..3 {
                if (x + y) % 2 == 0 {
                    src_grid.set(Pos::new(x, y), white).unwrap();
                }
            }
        }

        // Create a 5x5 destination buffer.
        let dst_grid = ArrayGrid::<Color<Rgba8888>, 25, RowMajor>::new_filled(5, 5, black);
        let mut pixel_buf = Framebuffer::from(dst_grid);
        pixel_buf.copy_rect(
            src_grid,
            Rect::from_tlbr(Pos::new(0, 0), Pos::new(3, 3)).unwrap(),
            Pos::new(1, 1),
        );

        #[rustfmt::skip]
        assert_eq!(
            pixel_buf.inner.into_inner().0,
            [
                black, black, black, black, black,
                black, white, black, white, black,
                black, black, white, black, black,
                black, white, black, white, black,
                black, black, black, black, black,
            ]
        );
    }
}
