#[macro_use]
extern crate diesel;
#[macro_use]
extern crate failure;
extern crate futures;
extern crate grpcio;
extern crate protobuf;
extern crate r2d2;
extern crate uuid;

mod db_schema;
mod error;
mod rpc;

use rpc::yacchauyo_grpc::Yacchauyo;
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};
use futures::Future;
use std::sync::Arc;
use diesel::prelude::*;

#[derive(Clone)]
struct Server {
    pool: r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>,
}

impl Server {
    fn new() -> Self {
        unimplemented!()
    }
}

impl Yacchauyo for Server {
    fn texts_index(
        &self,
        ctx: RpcContext,
        _req: ::rpc::yacchauyo::TextsQuery,
        sink: UnarySink<rpc::yacchauyo::Texts>
    ) {
        let mut response = ::rpc::yacchauyo::Texts::new();
        let mut first_text = ::rpc::yacchauyo::Text::new();
        first_text.set_title("Ethica more geometrico demonstrata".to_string());
        response.texts.push(first_text);
        let f = sink.success(response)
            .map_err(|_err| ());
        ctx.spawn(f);
    }
}

pub fn start() {
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
