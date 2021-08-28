//! # rich-sdl2-net-rust
//!
//! The SDL_net 2.0 wrapper for Rust.

#![warn(missing_docs)]

use rich_sdl2_rust::Sdl;
use std::marker::PhantomData;

mod bind;
pub mod client;
pub mod server;

/// A root controller for SDL2_net.
pub struct Net<'sdl> {
    _phantom: PhantomData<&'sdl Sdl>,
}

impl<'sdl> Net<'sdl> {
    /// Constructs a root controller with SDL2 controller.
    pub fn new(_sdl: &'sdl Sdl) -> Self {
        let ret = unsafe { bind::SDLNet_Init() };
        if ret != 0 {
            Sdl::error_then_panic("sdl_net init");
        }
        Self {
            _phantom: PhantomData,
        }
    }
}

impl Drop for Net<'_> {
    fn drop(&mut self) {
        unsafe { bind::SDLNet_Quit() }
    }
}
