//! Socket to create connection or send a datagram.

use rich_sdl2_rust::{Result, Sdl, SdlError};
use std::{marker::PhantomData, net::SocketAddrV4, os::raw::c_int, ptr::NonNull};

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
    _phantom: PhantomData<&'socket mut ()>,
}

impl<'socket> UdpSocket<'socket> {
    pub(crate) fn new(port: u16) -> Result<Self> {
        let ptr = unsafe { bind::SDLNet_UDP_Open(port) };
        if ptr.is_null() {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(Self {
                socket: NonNull::new(ptr).unwrap(),
                _phantom: PhantomData,
            })
        }
    }

    /// Returns the channels of the socket.
    pub fn channels(&self) -> Vec<UdpChannel> {
        const MAX_UDP_ADDRESSES: u32 = 4;
        (0..MAX_UDP_ADDRESSES)
            .map(|id| UdpChannel {
                id: id as _,
                socket: self,
            })
            .collect()
    }
}

impl Drop for UdpSocket<'_> {
    fn drop(&mut self) {
        unsafe { bind::SDLNet_UDP_Close(self.socket.as_ptr()) }
    }
}

/// A channel of a udp socket to matchup packets to specific clients easier.
#[derive(Clone)]
pub struct UdpChannel<'chan> {
    id: c_int,
    socket: &'chan UdpSocket<'chan>,
}

impl<'chan> UdpChannel<'chan> {
    /// Binds the socket address to the channel, or `Err` on failure.
    pub fn bind_channel(&self, address: SocketAddrV4) -> Result<()> {
        let address = bind::IPaddress {
            host: u32::from_ne_bytes(address.ip().octets()),
            port: address.port(),
        };
        let ret = unsafe {
            bind::SDLNet_UDP_Bind(self.socket.socket.as_ptr(), self.id, &address as *const _)
        };
        if ret < 0 {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(())
        }
    }

    /// Unbinds the socket address from the channel.
    pub fn unbind(&self) {
        unsafe { bind::SDLNet_UDP_Unbind(self.socket.socket.as_ptr(), self.id) }
    }
}
