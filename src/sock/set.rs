//! Socket management by a socket set.

use rich_sdl2_rust::Sdl;
use std::ptr::NonNull;

use super::{TcpSocket, UdpSocket};
use crate::{bind, Net};

enum GeneralSocket<'net> {
    Tcp(TcpSocket<'net>),
    Udp(UdpSocket<'net>),
}

/// A socket set to observe a socket.
pub struct SocketSet<'net> {
    ptr: NonNull<bind::_SDLNet_SocketSet>,
    sockets: Vec<GeneralSocket<'net>>,
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
            sockets: Vec::with_capacity(cap),
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

    /// Appends a tcp socket.
    pub fn push_tcp(&mut self, tcp: TcpSocket<'net>) {
        self.sockets.push(GeneralSocket::Tcp(tcp));
    }

    /// Appends a udp socket.
    pub fn push_udp(&mut self, udp: UdpSocket<'net>) {
        self.sockets.push(GeneralSocket::Udp(udp));
    }
}

impl Drop for SocketSet<'_> {
    fn drop(&mut self) {
        unsafe { bind::SDLNet_FreeSocketSet(self.ptr.as_ptr()) }
    }
}
