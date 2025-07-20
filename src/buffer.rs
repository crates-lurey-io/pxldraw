#[cfg(feature = "alloc")]
mod alloc;

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

impl<T, F> Framebuffer<T, F>
where
    F: Format,
{
    // TODO: Is this a good API?
    pub fn as_inner(&self) -> &T {
        &self.inner
    }
}
