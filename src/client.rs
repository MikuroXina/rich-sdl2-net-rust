//! Servers for creating the connections.

use rich_sdl2_rust::{Result, Sdl, SdlError};
use std::{
    ffi::CString,
    marker::PhantomData,
    mem::MaybeUninit,
    net::{Ipv4Addr, SocketAddrV4},
};

use crate::{bind, Net};

/// A client to create the connection.
pub struct NetClient<'net> {
    socket: SocketAddrV4,
    _phantom: PhantomData<&'net Net<'net>>,
}

impl<'net> NetClient<'net> {
    /// Constructs and ready to start the client socket.
    pub fn new(_net: &'net Net<'net>, address: Ipv4Addr, port: Option<u16>) -> Result<Self> {
        let address_cstr = CString::new(address.to_string()).unwrap();
        let mut address = MaybeUninit::uninit();
        let ret = unsafe {
            bind::SDLNet_ResolveHost(
                address.as_mut_ptr(),
                address_cstr.as_ptr(),
                port.unwrap_or(0),
            )
        };
        if ret != 0 {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            let address = unsafe { address.assume_init() };
            Ok(Self {
                socket: SocketAddrV4::new(Ipv4Addr::from(address.host), address.port),
                _phantom: PhantomData,
            })
        }
    }

    /// Constructs from the hostname and ready to start the client socket.
    pub fn with_hostname(_net: &'net Net<'net>, hostname: &str, port: Option<u16>) -> Result<Self> {
        let hostname_cstr = CString::new(hostname).unwrap();
        let mut address = MaybeUninit::uninit();
        let ret = unsafe {
            bind::SDLNet_ResolveHost(
                address.as_mut_ptr(),
                hostname_cstr.as_ptr(),
                port.unwrap_or(0),
            )
        };
        if ret != 0 {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            let address = unsafe { address.assume_init() };
            Ok(Self {
                socket: SocketAddrV4::new(Ipv4Addr::from(address.host), address.port),
                _phantom: PhantomData,
            })
        }
    }
}
