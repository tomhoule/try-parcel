extern crate chrono;
#[macro_use]
extern crate configure;
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate failure;
extern crate futures;
extern crate grpcio;
extern crate protobuf;
extern crate r2d2;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate uuid;

pub mod config;
pub mod db_schema;
pub mod error;
pub mod models;
pub mod rpc;
pub mod server;
pub mod test_utils;

use rpc::yacchauyo_grpc::Yacchauyo;
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};
use futures::Future;
use std::sync::Arc;
use server::Server;
use rpc::yacchauyo::*;

macro_rules! plug {
    ($name:ident, $req:ty, $res:ty) => {
        fn $name(
                &self,
                ctx: RpcContext,
                req: $req,
                sink: UnarySink<$res>
            ) {
                match self.$name(req) {
                    Ok(response) => {
                        let f = sink.success(response)
                            .map_err(|_err| ());
                        ctx.spawn(f)
                    }
                    Err(err) => {
                        let f = sink.fail(err.as_grpc())
                            .map_err(|_err| ());
                        ctx.spawn(f)
                    }
                };
            }
    }
}

impl Yacchauyo for Server {
    plug!(texts_index, TextsQuery, Texts);
}

pub fn start() {
    use_default_config!();
    let env = Arc::new(Environment::new(4));
    let service = ::rpc::yacchauyo_grpc::create_yacchauyo(Server::new());
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("127.0.0.1", 4443)
        .build()
        .unwrap();
    server.start();
    for &(ref host, port) in server.bind_addrs() {
        println!("listening on {}:{}", host, port);
    }
    // let (tx, rx) = oneshot::channel();
    // thread::spawn(move || {
    //     println!("Press ENTER to exit...");
    //     let _ = ::std::io::stdin().read(&mut [0]).unwrap();
    //     tx.send(())
    // });
    // let _ = rx.wait();
    loop {}
    let _ = server.shutdown().wait();
}
