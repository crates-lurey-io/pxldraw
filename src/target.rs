//! Defines a target for drawing pixels on a 2D surface.

use grixy::core::Pos;
use pxlfmt::pixel::{Format, Pixel};

/// A target for drawing pixels on a 2D surface.
///
/// This trait's methods are optimized for writing to an in-memory framebuffer.
pub trait DrawTarget {
    /// Color format used by the target.
    type Format: Format;

    /// Error that may occur during drawing operations.
    type Error;

    /// Draws a single pixel at the specified coordinates with the given color.
    ///
    /// When the coordinates are out of bounds, the pixel is not drawn.
    ///
    /// ## Errors
    ///
    /// Whether the operation may fail depends on the implementation.
    fn draw_pixel(&mut self, pos: Pos, color: Pixel<Self::Format>) -> Result<(), Self::Error>;
}
