extern crate bytes;
extern crate futures;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;
extern crate tokio_timer;

use std::io::{self, ErrorKind};
use std::str;
use std::time::Duration;

use bytes::BytesMut;

use futures::{future, Future, BoxFuture};

use tokio_io::codec::{Encoder, Decoder};
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use tokio_proto::multiplex::{ServerProto, RequestId};
use tokio_proto::TcpServer;
use tokio_service::Service;
use tokio_timer::Timer;

pub struct LineCodec(RequestId);

fn decode_u64(buf: &[u8]) -> Result<u64, io::Error> {
    str::from_utf8(buf)
        .map_err(|e| io::Error::new(ErrorKind::InvalidData, e))?
        .parse()
        .map_err(|e| io::Error::new(ErrorKind::InvalidData, e))
}

fn make_error(msg: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::Other, msg)
}

impl Decoder for LineCodec {
    type Item = (RequestId, (u64, String));
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<Self::Item>> {
        if let Some(i) = buf.iter().position(|&b| b == b'\n') {
            let mut line = buf.split_to(i);
            buf.split_to(1);

            let j = line.iter().position(|&b| b == b' ')
                .ok_or_else(|| make_error("no space in message"))?;

            let delay = line.split_to(j);
            line.split_to(1);

            let n = decode_u64(&delay)
                .map_err(|_| make_error("invalid delay"))?;

            let s = str::from_utf8(&line)
                .map_err(|_| make_error("invalid UTF-8"))?;

            let seq = self.0;
            self.0 += 1;

            Ok(Some((seq, (n, s.to_string()))))
        } else {
            Ok(None)
        }
    }
}

impl Encoder for LineCodec {
    type Item = (RequestId, String);
    type Error = io::Error;

    fn encode(&mut self, msg: Self::Item, buf: &mut BytesMut) -> io::Result<()> {
        // buf.extend(format!("SEQ({}): ", msg.0).as_bytes());
        buf.extend(msg.1.as_bytes());
        buf.extend(b"\n");
        Ok(())
    }
}

pub struct LineProto;

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for LineProto {
    type Request = (u64, String);
    type Response = String;

    type Transport = Framed<T, LineCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(LineCodec(0)))
    }
}

pub struct EchoService;

impl Service for EchoService {
    type Request = (u64, String);
    type Response = String;
    type Error = io::Error;
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn call(&self, (delay, msg): Self::Request) -> Self::Future {
        Timer::default()
            .sleep(Duration::from_millis(delay))
            .then(|_| future::ok(msg))
            .boxed()
    }
}

fn main() {
    let server = TcpServer::new(LineProto, "0.0.0.0:12345".parse().unwrap());

    server.serve(|| Ok(EchoService));
}
