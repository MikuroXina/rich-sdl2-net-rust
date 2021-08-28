//! Servers for receiving the connections.

use rich_sdl2_rust::{Result, Sdl, SdlError};
use std::{marker::PhantomData, mem::MaybeUninit, ptr::NonNull};

use crate::{bind, conn::TcpConnection, Net};

/// A server to serve the connection.
pub struct NetServer<'net> {
    address: bind::IPaddress,
    _phantom: PhantomData<&'net Net<'net>>,
}

impl<'net> NetServer<'net> {
    /// Constructs and ready to start the server socket.
    pub fn new(_net: &'net Net<'net>, port: u16) -> Result<Self> {
        let mut address = MaybeUninit::uninit();
        let ret = unsafe { bind::SDLNet_ResolveHost(address.as_mut_ptr(), std::ptr::null(), port) };
        if ret != 0 {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            let mut address = unsafe { address.assume_init() };
            address.port = port;
            Ok(Self {
                address,
                _phantom: PhantomData,
            })
        }
    }

    /// Opens a tcp connection socket.
    pub fn open_tcp(&'net mut self) -> Result<TcpSocket<'net>> {
        TcpSocket::new(self)
    }
}

/// A tcp connection socket for receive packets.
pub struct TcpSocket<'socket> {
    socket: NonNull<bind::_TCPsocket>,
    _phantom: PhantomData<&'socket mut NetServer<'socket>>,
}

impl<'socket> TcpSocket<'socket> {
    fn new(server: &'socket mut NetServer<'socket>) -> Result<Self> {
        let ptr = unsafe { bind::SDLNet_TCP_Open(&mut server.address as *mut _) };
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
