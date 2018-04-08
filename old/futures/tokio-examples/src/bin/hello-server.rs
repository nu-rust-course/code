extern crate futures;
extern crate tokio_core;
extern crate tokio_io;

use futures::Future;
use futures::stream::Stream;
use tokio_core::reactor::Core;
use tokio_core::net::TcpListener;

fn main() {
    let mut core = Core::new().unwrap();
    let address = "0.0.0.0:12345".parse().unwrap();
    let listener = TcpListener::bind(&address, &core.handle()).unwrap();

    let connections = listener.incoming();
    let handle = core.handle();
    let welcomes = connections.and_then(|(socket, _peer_addr)| {
        let serve_one =
            tokio_io::io::write_all(socket, b"Hello, world!\n")
                .then(|_| Ok(()));
        handle.spawn(serve_one);
        Ok(())
//        tokio_io::io::write_all(socket, b"Hello, world!\n")
    });
    let server = welcomes.for_each(|_| {
        Ok(())
    });

    core.run(server).unwrap();
}

