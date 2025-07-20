#![cfg_attr(
    not(feature = "alloc"),
    compile_error("The `alloc` crate feature must be enabled for this module.")
)]
extern crate alloc;

use core::marker::PhantomData;

use crate::{
    buffer::Framebuffer,
    core::{Color, Format},
};
use grixy::{buf::VecGrid, core::RowMajor};

impl<F> Framebuffer<VecGrid<Color<F>, RowMajor>, F>
where
    F: Format,
{
    #[must_use]
    pub fn new(width: usize, height: usize) -> Self {
        let inner = VecGrid::new(width, height);
        Self {
            inner,
            _fmt: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        use pxlfmt::prelude::Rgba8888;
        let fb = Framebuffer::<_, Rgba8888>::new(100, 100);
        assert_eq!(fb.as_ref().as_ref().len(), 100 * 100);
    }
}
