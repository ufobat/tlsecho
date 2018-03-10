extern crate openssl;
extern crate tokio;
extern crate tokio_openssl;
extern crate tokio_uds;

use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod, SslStream};
use std::sync::Arc;
// use std::thread;
use tokio::net::{TcpListener, TcpStream};
use tokio_uds::UnixStream;
use tokio::prelude::*;
use tokio_openssl::{SslAcceptorExt, AcceptAsync};
use tokio::reactor::Handle;

fn main() {
    let listening_addr = "0.0.0.0:8443".parse().unwrap();
    let listener = TcpListener::bind(&listening_addr).unwrap();

    let acceptor = Arc::new(get_acceptor());
    let server = listener.incoming().for_each(
        move |stream| {
            // let acceptor = acceptor.clone(); // theads
            let stream = acceptor.accept_async(stream); // Future

            handle_client(stream);

            Ok(())
        }
    ).map_err(
        |err| {
            println!("error while listening: {:?}", err);
        }
    );
    tokio::run(server);
}

fn handle_client(stream: AcceptAsync<tokio::net::TcpStream>) {
    // each stream into a own thread?

    let promise = stream.and_then(|tls_stream| {
        let (reader, writer) = tls_stream.split();
        let handle = Handle::current(); // for the UNIX sockets
        let uds = UnixStream::connect("/home/martin/uds", &handle).unwrap();
        Ok(())
    }).map_err(
        |err| {
            println!("error while accepting: {:?}", err);
        }
    );
    // unix socket öffnen
    tokio::spawn(promise);
}

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

