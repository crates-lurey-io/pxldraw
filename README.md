# pxldraw

2D software pixel renderer.

[![Test](https://github.com/crates-lurey-io/pxldraw/actions/workflows/test.yml/badge.svg)](https://github.com/crates-lurey-io/pxldraw/actions/workflows/test.yml)
[![Docs](https://github.com/crates-lurey-io/pxldraw/actions/workflows/docs.yml/badge.svg)](https://github.com/crates-lurey-io/pxldraw/actions/workflows/docs.yml)
[![Crates.io Version](https://img.shields.io/crates/v/pxldraw)](https://crates.io/crates/pxldraw)
[![codecov](https://codecov.io/gh/crates-lurey-io/pxldraw/graph/badge.svg?token=Z3VUWA3WYY)](https://codecov.io/gh/crates-lurey-io/pxldraw)

The `pxldraw` crate plays a similar role to [`embedded-graphics-core`][], but
is built specifically for software[^1] rendering of pixels on to a `DrawTarget`;
this crate also provides a `Framebuffer` type that can be used to implement
in-memory 2D draw operations on a grid of pixels.

[`embedded-graphics-core`]: https://crates.io/crates/embedded-graphics-core

[^1]: `DrawTarget` could be implemented in a hardware accelerated way by another
      crate.

## Contributing

This project uses [`just`][] to run commands the same way as the CI:

- `cargo just check` to check formatting and lints.
- `cargo just coverage` to generate and preview code coverage.
- `cargo just doc` to generate and preview docs.
- `cargo just test` to run tests.

[`just`]: https://crates.io/crates/just

For a full list of commands, see the [`Justfile`](./Justfile).
