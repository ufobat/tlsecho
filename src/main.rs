extern crate futures;
extern crate openssl;
extern crate tokio_core;
extern crate tokio_openssl;

use futures::{Future, Stream};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod, SslStream};
use std::sync::Arc;
use std::thread;
use tokio_core::net::TcpListener;
use tokio_core::reactor::Core;
use tokio_openssl::SslAcceptorExt;

fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let listening_addr = "0.0.0.0:8443".parse().unwrap();
    let listener = TcpListener::bind(&listening_addr, &handle).unwrap();

    let acceptor = Arc::new(get_acceptor());
    let server = listener.incoming().for_each(
        |(stream, _remote_addr)| {
            let acceptor = acceptor.clone();
            handle.spawn(move || {
                let stream = acceptor.accept(stream).unwrap();
                // handle_client(stream);
            });
        }
    );
    core.run(server).unwrap();
}

// fn handle_client(stream: SslStream<TcpStream>) {
// }

fn get_acceptor() -> SslAcceptor {
    // change this to use specific configurations
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder.set_private_key_file("key.pem", SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file("certs.pem").unwrap();
    builder.check_private_key().unwrap();
    // builder.set_tmp_ecdh();
    // builder.set_verify_cb(callback_function);
    builder.build()
}

