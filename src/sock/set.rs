//! Socket management by a socket set.

use rich_sdl2_rust::Sdl;
use std::ptr::NonNull;

use super::{TcpSocket, UdpSocket};
use crate::{bind, Net};

enum GeneralSocket<'net> {
    Tcp(TcpSocket<'net>),
    Udp(UdpSocket<'net>),
}

impl PartialEq for GeneralSocket<'_> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Tcp(l0), Self::Tcp(r0)) => l0.socket == r0.socket,
            (Self::Udp(l0), Self::Udp(r0)) => l0.socket == r0.socket,
            _ => false,
        }
    }
}

impl PartialEq<TcpSocket<'_>> for GeneralSocket<'_> {
    fn eq(&self, other: &TcpSocket<'_>) -> bool {
        match self {
            Self::Tcp(l0) => l0.socket == other.socket,
            _ => false,
        }
    }
}

impl PartialEq<UdpSocket<'_>> for GeneralSocket<'_> {
    fn eq(&self, other: &UdpSocket<'_>) -> bool {
        match self {
            Self::Udp(l0) => l0.socket == other.socket,
            _ => false,
        }
    }
}

/// A socket set to observe a socket.
pub struct SocketSet<'net> {
    ptr: NonNull<bind::_SDLNet_SocketSet>,
    sockets: Vec<GeneralSocket<'net>>,
}

impl<'set, 'net: 'set> SocketSet<'set> {
    /// Constructs a new socket set.
    pub fn new(_net: &'net Net<'net>) -> Self {
        Self {
            ptr: NonNull::dangling(),
            sockets: Vec::new(),
        }
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
    pub fn active_sockets_num(&self, timeout: u32) -> usize {
        let ret = unsafe { bind::SDLNet_CheckSockets(self.ptr.as_ptr(), timeout) };
        if ret < 0 {
            Sdl::error_then_panic("get active sockets");
        }
        ret as usize
    }

    /// Reserves capacity for at least `additional` more elements.
    pub fn reserve(&mut self, additional: usize) {
        if self.sockets.capacity() != 0 {
            unsafe { bind::SDLNet_FreeSocketSet(self.ptr.as_ptr()) }
        }
        self.sockets.reserve(additional);
        let ptr = unsafe { bind::SDLNet_AllocSocketSet(self.sockets.capacity() as _) };
        self.ptr = NonNull::new(ptr).unwrap_or_else(|| Sdl::error_then_panic("alloc socket set"));
    }

    /// Appends a tcp socket.
    pub fn push_tcp<'socket>(&mut self, tcp: TcpSocket<'socket>)
    where
        'net: 'socket,
        'socket: 'set,
    {
        if self.sockets.len() == self.sockets.capacity() {
            self.reserve(1);
        }
        let _ = unsafe { bind::SDLNet_AddSocket(self.ptr.as_ptr(), tcp.socket.as_ptr().cast()) };
        self.sockets.push(GeneralSocket::Tcp(tcp));
    }

    /// Removes a tcp socket.
    pub fn remove_tcp(&mut self, tcp: &TcpSocket) {
        if let Some(found) = self.sockets.iter().enumerate().position(|(_, e)| e == tcp) {
            if let GeneralSocket::Tcp(found) = &self.sockets[found] {
                let _ = unsafe {
                    bind::SDLNet_DelSocket(self.ptr.as_ptr(), found.socket.as_ptr().cast())
                };
            }
            self.sockets.remove(found);
        }
    }

    /// Appends a udp socket.
    pub fn push_udp<'socket>(&mut self, udp: UdpSocket<'socket>)
    where
        'net: 'socket,
        'socket: 'set,
    {
        if self.sockets.len() == self.sockets.capacity() {
            self.reserve(1);
        }
        let _ = unsafe { bind::SDLNet_AddSocket(self.ptr.as_ptr(), udp.socket.as_ptr().cast()) };
        self.sockets.push(GeneralSocket::Udp(udp));
    }

    /// Removes a tcp socket.
    pub fn remove_udp(&mut self, tcp: &UdpSocket) {
        if let Some(found) = self.sockets.iter().enumerate().position(|(_, e)| e == tcp) {
            if let GeneralSocket::Udp(found) = &self.sockets[found] {
                let _ = unsafe {
                    bind::SDLNet_DelSocket(self.ptr.as_ptr(), found.socket.as_ptr().cast())
                };
            }
            self.sockets.remove(found);
        }
    }
}

impl Drop for SocketSet<'_> {
    fn drop(&mut self) {
        unsafe { bind::SDLNet_FreeSocketSet(self.ptr.as_ptr()) }
    }
}
