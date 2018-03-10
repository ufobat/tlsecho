extern crate openssl;
extern crate tokio;
extern crate tokio_openssl;

use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod, SslStream};
use std::sync::Arc;
use std::thread;
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;
use tokio_openssl::{SslAcceptorExt, AcceptAsync};

fn main() {
    let listening_addr = "0.0.0.0:8443".parse().unwrap();
    let listener = TcpListener::bind(&listening_addr).unwrap();

    let acceptor = Arc::new(get_acceptor());
    let server = listener.incoming().for_each(
        move |stream| {
            let acceptor = acceptor.clone();
            let stream = acceptor.accept_async(stream); // Future

            // each stream into a own thread?
            // thread::spawn(move || handle_client(stream) );

            Ok(())
        }
    ).map_err(
        |err| {
            println!("error while accepting: {:?}", err);
        }
    );
    tokio::run(server);
}

fn handle_client(stream: AcceptAsync<tokio::net::TcpStream>) {

    // unix socket öffnen
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

