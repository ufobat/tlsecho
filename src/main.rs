extern crate openssl;
extern crate tokio;
extern crate tokio_openssl;
extern crate tokio_uds;

use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::sync::Arc;
// use std::thread;
use tokio::net::TcpListener;
use tokio::io;
use tokio_uds::UnixStream;
use tokio::prelude::*;
use tokio_openssl::{SslAcceptorExt, AcceptAsync};

fn main() {
    let listening_addr = "0.0.0.0:8443".parse().unwrap();
    let listener = TcpListener::bind(&listening_addr).unwrap();

    let acceptor = Arc::new(get_acceptor());
    let server = listener.incoming().for_each(
        move |stream| {
            // TODO: accepting invokes the verify_callback
            // so this should be in a own thread
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

    let promise = stream.and_then(|tls_stream| {
        let (tls_reader, tls_writer) = tls_stream.split();
        let uds = UnixStream::connect("/home/martin/uds").unwrap();
        let (uds_reader, uds_writer) = uds.split();
        let sender = io::copy(tls_reader, uds_writer).map(|_| {
            // nothing
        }).map_err(
                |err| { println!("IO Err: {}", err) }
        );
        let receiver = io::copy(uds_reader, tls_writer).map(|_| {
            // nothing
        }).map_err(
            |err| { println!("IO Err: {}", err) }
        );
        // TODO? on the threadpool
        tokio::spawn(sender);
        tokio::spawn(receiver);
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

