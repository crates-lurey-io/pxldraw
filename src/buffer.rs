//! Pixel buffer.

#[cfg(feature = "alloc")]
mod inner_alloc;

mod impl_target;

use core::marker::PhantomData;
use pxlfmt::{formats::rgba::Rgba8888, pixel::Format};

/// An in-memory buffer for reading and writing pixels.
pub struct Framebuffer<T, F = Rgba8888>
where
    F: Format,
{
    inner: T,
    _fmt: PhantomData<F>,
}

impl<T, F> AsRef<T> for Framebuffer<T, F>
where
    F: Format,
{
    fn as_ref(&self) -> &T {
        &self.inner
    }
}

impl<T, F> AsMut<T> for Framebuffer<T, F>
where
    F: Format,
{
    fn as_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}
