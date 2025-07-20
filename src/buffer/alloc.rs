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
