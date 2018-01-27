#[macro_use]
extern crate configure;
extern crate grpcio;
extern crate yacchauyo;
extern crate diesel;
extern crate futures;

use futures::Future;
use yacchauyo::rpc::yacchauyo_grpc::YacchauyoClient;
use yacchauyo::server::Server;
use std::sync::Arc;
use yacchauyo::rpc::yacchauyo as proto;
use grpcio::{ServerBuilder, Environment};
use std::panic::UnwindSafe;
use diesel::prelude::*;
use yacchauyo::models;

fn make_client() -> YacchauyoClient {
    let env = Arc::new(grpcio::Environment::new(2));
    let channel = grpcio::ChannelBuilder::new(env).connect("127.0.0.1:5555");
     YacchauyoClient::new(channel)
}

fn make_conn() -> PgConnection {
    let database_url = ::std::env::var("YACCHAUYO_DATABASE_URL").unwrap();
    PgConnection::establish(&database_url).expect("can connect to pg")
}

fn e2e_test<Inner: UnwindSafe + FnOnce() -> ()>(inner: Inner) {
    ::std::env::set_var("YACCHAUYO_DATABASE_URL", "postgres://postgres@localhost:5432/yacchauyo_test");
    use_default_config!();

    let env = Arc::new(Environment::new(4));
    let service = yacchauyo::rpc::yacchauyo_grpc::create_yacchauyo(Server::new());
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("127.0.0.1", 5555)
        .build()
        .unwrap();
    server.start();


    let result = ::std::panic::catch_unwind(move || {
        inner()
    });

    server.shutdown().wait().unwrap();

    assert!(result.is_ok());
}

#[test]
fn texts_index_returns_texts() {
    e2e_test(|| {
        let client = make_client();
        let req = proto::TextsQuery::new();
        let conn = make_conn();
        let text = models::texts::NewText {
            title: "Abc".to_string(),
            slug: "abc".to_string(),
            authors: "Calvin & Hobbes".to_string(),
            description: None,
        };
        text.save(&conn).unwrap();
        let res = client.texts_index(&req)
            .unwrap();
        assert_eq!(res.texts.len(), 1);
        assert_eq!(res.texts[0].title, "Abc");
    })
}
