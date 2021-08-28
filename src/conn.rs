//! Connections between client and server.

use std::{marker::PhantomData, ptr::NonNull};

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
}

impl Drop for TcpConnection<'_> {
    fn drop(&mut self) {
        unsafe { bind::SDLNet_TCP_Close(self.opponent.as_ptr()) }
    }
}
