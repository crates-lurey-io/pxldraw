//! 2D software pixel renderer.
//!
//! The `pxldraw` crate plays a similar role to [`embedded-graphics-core`][], but is built
//! specifically for software[^1] rendering of pixels on to a [`DrawTarget`]; this crate also
//! provides a [`Framebuffer`] type that can be used to implement in-memory 2D draw operations on a
//! grid of pixels.
//!
//! [`DrawTarget`]: `crate::target::DrawTarget`
//! [`embedded-graphics-core`]: https://crates.io/crates/embedded-graphics-core
//! [^1]: `DrawTarget` could be implemented in a hardware accelerated way by another crate.

#![no_std]

pub mod target;

use core::convert::Infallible;

pub use grixy::core::{Pos, Rect};
use grixy::grid::GridWrite;
pub use pxlfmt::pixel::Pixel as Color;
use pxlfmt::{pixel::Format, prelude::Rgba8888};

use crate::target::DrawTarget;

/// A framebuffer.
///
/// Any target that can implements [`GridWrite<Element = Color>`] is a valid framebuffer.
///
/// ## Examples
///
/// ```rust
/// use grixy::buf::ArrayGrid;
/// use pxldraw::{Color, target::DrawTarget, Framebuffer, Pos};
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

impl<T, F> DrawTarget for Framebuffer<T, F>
where
    F: Format,
    T: GridWrite<Element = Color<F>>,
{
    type Format = F;

    type Error = Infallible;

    fn draw_pixel(&mut self, pos: Pos, color: Color<F>) -> Result<(), Self::Error> {
        let _ = self.inner.set(pos, color);
        Ok(())
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
        let _ = pixel_buf.draw_pixel(Pos::new(0, 0), Color::<Rgba8888>::new(0xFFFF_FFFF));

        assert_eq!(
            pixel_buf.inner.get(Pos::new(0, 0)),
            Some(&Color::<Rgba8888>::new(0xFFFF_FFFF))
        );
    }
}
