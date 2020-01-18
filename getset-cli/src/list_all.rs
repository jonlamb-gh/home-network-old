use log::info;
use params::{GetSetFrame, GetSetOp, GetSetPayloadType, Request};
use std::io;
use std::io::prelude::*;
use std::net::SocketAddr;
use std::net::TcpStream;

pub fn list_all(address: SocketAddr) -> io::Result<()> {
    info!("Listing all parameters at {}", address);
    let mut buf: Vec<u8> = vec![0; 1500];

    let mut frame = GetSetFrame::new_unchecked(&mut buf[..]);
    let req = Request::new(0, 0, GetSetOp::ListAll, GetSetPayloadType::None);
    req.emit(&mut frame).unwrap();
    let wire_size = req.wire_size();

    let mut stream = TcpStream::connect(address)?;

    info!("Sending {} bytes : {}", wire_size, req);
    stream.write(&buf[..wire_size])?;

    Ok(())
}