//! Socket management by a socket set.

use rich_sdl2_rust::Sdl;
use std::{marker::PhantomData, ptr::NonNull};

use crate::{bind, Net};

/// A socket set to observe a socket.
pub struct SocketSet<'net> {
    ptr: NonNull<bind::_SDLNet_SocketSet>,
    _phantom: PhantomData<&'net ()>,
}

impl<'net> SocketSet<'net> {
    /// Constructs a new socket set.
    pub fn new(net: &'net Net<'net>) -> Self {
        Self::with_capacity(net, 1)
    }

    /// Constructs a new socket set with the capacity.
    pub fn with_capacity(_net: &'net Net<'net>, cap: usize) -> Self {
        let ptr = unsafe { bind::SDLNet_AllocSocketSet(cap as _) };
        Self {
            ptr: NonNull::new(ptr).unwrap_or_else(|| Sdl::error_then_panic("alloc socket set")),
            _phantom: PhantomData,
        }
    }

    /// Returns the number of the active sockets in the socket set.
    pub fn active_sockets(&self, timeout: u32) -> usize {
        let ret = unsafe { bind::SDLNet_CheckSockets(self.ptr.as_ptr(), timeout) };
        if ret < 0 {
            Sdl::error_then_panic("get active sockets");
        }
        ret as usize
    }
}

impl Drop for SocketSet<'_> {
    fn drop(&mut self) {
        unsafe { bind::SDLNet_FreeSocketSet(self.ptr.as_ptr()) }
    }
}
