#[macro_use]
extern crate configure;
extern crate diesel;
extern crate futures;
extern crate grpcio;
extern crate yacchauyo;

use futures::Future;
use yacchauyo::rpc::yacchauyo_grpc::YacchauyoClient;
use yacchauyo::server::Server;
use std::sync::Arc;
use yacchauyo::rpc::yacchauyo as proto;
use grpcio::{Environment, ServerBuilder};
use std::panic::UnwindSafe;

fn make_client() -> YacchauyoClient {
    let env = Arc::new(grpcio::Environment::new(2));
    let channel = grpcio::ChannelBuilder::new(env).connect("127.0.0.1:5555");
    YacchauyoClient::new(channel)
}

fn e2e_test<Inner: UnwindSafe + FnOnce() -> ()>(inner: Inner) {
    use_default_config!();

    let env = Arc::new(Environment::new(4));
    let service = yacchauyo::rpc::yacchauyo_grpc::create_yacchauyo(Server::new());
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("127.0.0.1", 5555)
        .build()
        .unwrap();
    server.start();

    let result = ::std::panic::catch_unwind(move || inner());

    server.shutdown().wait().unwrap();

    assert!(result.is_ok());
}

#[test]
fn text_creation_and_retrieval() {
    e2e_test(|| {
        let client = make_client();

        let mut req = proto::Text::new();
        req.set_title("Batman".to_string());
        req.set_authors("someone no doubt".to_string());
        req.set_slug("batman".to_string());

        client.create_text(&req).unwrap();

        let req = proto::TextsQuery::new();
        let res = client.texts_index(&req).unwrap();
        assert!(res.texts.iter().any(|text| text.title == "Batman"))
    })
}
