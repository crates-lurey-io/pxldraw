use crate::{buffer::Framebuffer, core::Color, target::DrawTarget};
use core::convert::Infallible;
use grixy::grid::GridWrite;
use pxlfmt::pixel::Format;

/// Implements [`DrawTarget`] by delegating to [`GridWrite`].
impl<T, F> DrawTarget for Framebuffer<T, F>
where
    T: GridWrite<Element = Color<F>>,
    F: Format,
{
    type Format = F;

    type Error = Infallible;

    fn draw_pixel(
        &mut self,
        pos: grixy::core::Pos,
        color: pxlfmt::prelude::Pixel<Self::Format>,
    ) -> Result<(), Self::Error> {
        let _ = self.inner.set(pos, color);
        Ok(())
    }

    fn fill_rect(
        &mut self,
        rect: grixy::core::Rect,
        color: Color<Self::Format>,
    ) -> Result<(), Self::Error> {
        self.inner.fill_rect_solid(rect, color);
        Ok(())
    }

    fn fill_rect_iter(
        &mut self,
        rect: grixy::core::Rect,
        pixels: impl IntoIterator<Item = Color<Self::Format>>,
    ) -> Result<(), Self::Error> {
        self.inner.fill_rect_iter(rect, pixels);
        Ok(())
    }
}
