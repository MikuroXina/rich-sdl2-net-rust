//! Servers for receiving the connections.

use std::{
    marker::PhantomData,
    mem::MaybeUninit,
    net::{Ipv4Addr, SocketAddrV4},
};

use rich_sdl2_rust::{Result, Sdl, SdlError};

use crate::{bind, Net};

/// A server to serve the connection.
pub struct NetServer<'net> {
    socket: SocketAddrV4,
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
            let address = unsafe { address.assume_init() };
            Ok(Self {
                socket: SocketAddrV4::new(Ipv4Addr::from(address.host), port),
                _phantom: PhantomData,
            })
        }
    }
}
