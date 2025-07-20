//! Defines a target for drawing pixels on a 2D surface.

use crate::core::{Pos, Rect};
use pxlfmt::pixel::{Format, Pixel};

/// A target for drawing pixels on a 2D surface.
///
/// ## Implementation
///
/// This trait's methods are optimized for writing to an in-memory framebuffer using CPU operations
/// but could also be accelerated by hardware where available; for example, a GPU or a specific
/// set of graphics instructions could optimize a `fill_rect` operation instead of iterating over
/// each pixel.
pub trait DrawTarget {
    /// Color format used by the target.
    type Format: Format;

    /// Errors occur when the drawing operation fails.
    ///
    /// Errors are intended to be used for operations that may fail, such as when the target is
    /// unavailable. A position being out of bounds is not considered an error, and the
    /// implementation should handle it gracefully by not drawing the pixel; for example, by
    /// ignoring the operation or clipping it to the bounds of the target.
    ///
    /// Some targets may use a buffered approach, where the drawing operations are queued and
    /// executed later, in which case `type Error = core::convert::Infallible` is appropriate as
    /// the operations are guaranteed to succeed until the buffer is flushed.
    type Error;

    /// Draws a single pixel at the specified coordinates with the given color.
    ///
    /// When the coordinates are out of bounds, the pixel is not drawn.
    ///
    /// ## Errors
    ///
    /// Whether the operation may fail depends on the implementation; see [`DrawTarget::Error`].
    fn draw_pixel(&mut self, pos: Pos, color: Pixel<Self::Format>) -> Result<(), Self::Error>;

    /// Fills a rectangular region, mutally exclusive of the right and bottom edges, with the
    /// specified color.
    ///
    /// Coordinates outside the target's bounds are ignored.
    ///
    /// ## Implementation
    ///
    /// By default invokes [`DrawTarget::draw_pixel`] for each pixel; override to optimize.
    ///
    /// ## Errors
    ///
    /// Whether the operation may fail depends on the implementation; see [`DrawTarget::Error`].
    fn fill_rect(&mut self, rect: Rect, color: Pixel<Self::Format>) -> Result<(), Self::Error> {
        let mut pixels = rect.into_iter_row_major();
        pixels.try_for_each(|pos| self.draw_pixel(pos, color))?;
        Ok(())
    }

    /// Fills a rectangular region, mutually exclusive of the right and bottom edges, with the
    /// specified colors.
    ///
    /// Coordinates outside the target's bounds are ignored. If the iterator provides fewer elements
    /// than the number of pixels in the rectangle, the remaining pixels are not drawn, and if it
    /// provides more, the excess elements are ignored.
    ///
    /// ## Implementation
    ///
    /// By default invokes [`DrawTarget::draw_pixel`] for each pixel; override to optimize.
    ///
    /// ## Errors
    ///
    /// Whether the operation may fail depends on the implementation; see [`DrawTarget::Error`].
    fn fill_rect_iter(
        &mut self,
        rect: Rect,
        pixels: impl IntoIterator<Item = Pixel<Self::Format>>,
    ) -> Result<(), Self::Error> {
        rect.into_iter_row_major()
            .zip(pixels)
            .try_for_each(|(pos, color)| self.draw_pixel(pos, color))?;
        Ok(())
    }
}
