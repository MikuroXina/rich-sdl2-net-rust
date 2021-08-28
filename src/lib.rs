//! # rich-sdl2-net-rust
//!
//! The SDL_net 2.0 wrapper for Rust.

#![warn(missing_docs)]

use rich_sdl2_rust::Sdl;
use std::{ffi::CStr, marker::PhantomData, net::Ipv4Addr};

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

    /// Resolves the ipv4 address to the hostname.
    pub fn resolve_ipv4(addr: Ipv4Addr) -> String {
        let address = bind::IPaddress {
            host: u32::from_ne_bytes(addr.octets()),
            port: 0,
        };
        let cstr = unsafe { CStr::from_ptr(bind::SDLNet_ResolveIP(&address as *const _)) };
        cstr.to_string_lossy().to_string()
    }
}

impl Drop for Net<'_> {
    fn drop(&mut self) {
        unsafe { bind::SDLNet_Quit() }
    }
}
