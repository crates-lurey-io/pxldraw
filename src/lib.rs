//! 2D software pixel renderer.
//!
//! The `pxldraw` crate plays a similar role to [`embedded-graphics-core`][], but is built
//! specifically for software[^1] rendering of pixels on to a [`DrawTarget`]; this crate also
//! provides a [`Framebuffer`][] type that can be used to implement in-memory 2D draw operations on a
//! grid of pixels.
//!
//! [`DrawTarget`]: `crate::target::DrawTarget`
//! [`Framebuffer`]: `crate::buffer::Framebuffer`
//! [`embedded-graphics-core`]: https://crates.io/crates/embedded-graphics-core
//! [^1]: `DrawTarget` could be implemented in a hardware accelerated way by another crate.
//!
//! ## Features
//!
//! _No features are enabled by default._
//!
//! ### `alloc`
//!
//! Enables the `alloc` crate, including creating `alloc::vec::Vec`-backed framebuffers
//!
//! ### `bytemuck`
//!
//! Enables cheap "view as bytes" (`&[u8]`) conversions on dependencies.

#![no_std]

pub mod buffer;
pub mod core;
pub mod target;
