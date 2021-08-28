//! Connections between client and server.

use std::{
    marker::PhantomData,
    net::{Ipv4Addr, SocketAddrV4},
    ptr::NonNull,
};

use crate::bind;

/// A tcp connection.
pub struct TcpConnection<'req> {
    opponent: NonNull<bind::_TCPsocket>,
    _phantom: PhantomData<&'req ()>,
}

impl<'req> TcpConnection<'req> {
    pub(crate) fn new(opponent: NonNull<bind::_TCPsocket>) -> Self {
        Self {
            opponent,
            _phantom: PhantomData,
        }
    }

    /// Returns the socket address of the connected party.
    pub fn address(&self) -> SocketAddrV4 {
        let addr = unsafe { &*bind::SDLNet_TCP_GetPeerAddress(self.opponent.as_ptr()) };
        SocketAddrV4::new(Ipv4Addr::from(addr.host), addr.port)
    }
}

impl Drop for TcpConnection<'_> {
    fn drop(&mut self) {
        unsafe { bind::SDLNet_TCP_Close(self.opponent.as_ptr()) }
    }
}
