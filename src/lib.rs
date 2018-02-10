#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate askama;
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
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[cfg(test)]
#[macro_use]
extern crate serde_json;
#[cfg(not(test))]
extern crate serde_json;
extern crate uuid;

pub mod config;
pub mod db_schema;
pub mod error;
pub mod models;
pub mod rpc;
#[cfg(test)]
pub mod test_utils;
pub mod utils;
pub mod web;

pub fn start_rpc() {
    rpc::start();
}

pub fn start_web() {
    web::start::start().launch();
}
