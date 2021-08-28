//! Socket to create connection or send a datagram.

use rich_sdl2_rust::{Result, Sdl, SdlError};
use std::{marker::PhantomData, ptr::NonNull};

use crate::{bind, conn::TcpConnection};

/// A tcp connection socket for receive packets.
pub struct TcpSocket<'socket> {
    socket: NonNull<bind::_TCPsocket>,
    _phantom: PhantomData<&'socket mut ()>,
}

impl<'socket> TcpSocket<'socket> {
    pub(crate) fn new(address: &'socket mut bind::IPaddress) -> Result<Self> {
        let ptr = unsafe { bind::SDLNet_TCP_Open(address as *mut _) };
        if ptr.is_null() {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(Self {
                socket: NonNull::new(ptr).unwrap(),
                _phantom: PhantomData,
            })
        }
    }

    /// Polls a request from a client, or `None` if no requests received.
    pub fn try_ack(&'socket self) -> Option<TcpConnection<'socket>> {
        let opponent = unsafe { bind::SDLNet_TCP_Accept(self.socket.as_ptr()) };
        NonNull::new(opponent).map(TcpConnection::new)
    }
}

impl Drop for TcpSocket<'_> {
    fn drop(&mut self) {
        unsafe { bind::SDLNet_TCP_Close(self.socket.as_ptr()) }
    }
}

/// A udp connection socket for send or receive packets.
pub struct UdpSocket<'socket> {
    socket: NonNull<bind::_UDPsocket>,
    address: &'socket mut bind::IPaddress,
}

impl<'socket> UdpSocket<'socket> {
    pub(crate) fn new(address: &'socket mut bind::IPaddress) -> Result<Self> {
        let ptr = unsafe { bind::SDLNet_UDP_Open(address.port) };
        if ptr.is_null() {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(Self {
                socket: NonNull::new(ptr).unwrap(),
                address,
            })
        }
    }
}
