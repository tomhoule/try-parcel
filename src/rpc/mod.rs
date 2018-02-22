pub mod yacchauyo;
pub mod yacchauyo_grpc;
pub mod server;

use futures::Future;
use futures::sync::oneshot;
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};
use rpc::yacchauyo_grpc::Yacchauyo;
use rpc::yacchauyo::*;
use rpc::server::Server;
use std::io::prelude::*;
use std::sync::Arc;
use std::thread;

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
    plug!(create_text, Text, Text);
    plug!(query_fragments, FragmentsQuery, FragmentsQuery);
    plug!(patch_text, Text, Text);
    plug!(text_schema, TextsQuery, Schema);
    plug!(patch_schema, Schema, Schema);
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
    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        println!("Press ENTER to exit...");
        let _ = ::std::io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });
    let _ = rx.wait();
    let _ = server.shutdown().wait();
}
