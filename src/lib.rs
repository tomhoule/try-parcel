extern crate futures;
extern crate grpcio;
// extern crate grpcio_proto;
extern crate protobuf;

mod rpc;

use rpc::yacchauyo_grpc::Yacchauyo;
use std::io::prelude::*;
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};
use futures::Future;
use std::sync::Arc;
use futures::sync::oneshot;
use std::thread;

#[derive(Clone)]
struct Server;

impl Yacchauyo for Server {
    fn texts_index(
        &self,
        ctx: ::grpcio::RpcContext,
        _req: ::rpc::yacchauyo::TextsQuery,
        sink: ::grpcio::UnarySink<rpc::yacchauyo::Texts>
    ) {
        let mut response = ::rpc::yacchauyo::Texts::new();
        let mut first_text = ::rpc::yacchauyo::Text::new();
        first_text.set_title("Ethica more geometrico demonstrata".to_string());
        response.texts.push(first_text);
        let f = sink.success(response)
            .map_err(|err| panic!("{:?}", err));
        ctx.spawn(f);
    }
}

pub fn start() {
    let env = Arc::new(Environment::new(4));
    let service = ::rpc::yacchauyo_grpc::create_yacchauyo(Server);
    println!("Got this far 97");
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("127.0.0.1", 4443)
        .build()
        .unwrap();
    server.start();
    println!("Got thi far 99");
    // for &(ref host, port) in server.bind_addrs() {
    //     println!("listening on {}:{}", host, port);
    // }
    // let (tx, rx) = oneshot::channel();
    // thread::spawn(move || {
    //     println!("Press ENTER to exit...");
    //     let _ = ::std::io::stdin().read(&mut [0]).unwrap();
    //     tx.send(())
    // });
    // let _ = rx.wait();
    loop {}
    println!("Got this far 101");
    let _ = server.shutdown().wait();
}
