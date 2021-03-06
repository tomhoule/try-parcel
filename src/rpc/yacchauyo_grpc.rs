// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_YACCHAUYO_TEXTS_INDEX: ::grpcio::Method<super::yacchauyo::TextsQuery, super::yacchauyo::Texts> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/Yacchauyo/TextsIndex",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_YACCHAUYO_CREATE_TEXT: ::grpcio::Method<super::yacchauyo::Text, super::yacchauyo::Text> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/Yacchauyo/CreateText",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_YACCHAUYO_PATCH_TEXT: ::grpcio::Method<super::yacchauyo::Text, super::yacchauyo::Text> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/Yacchauyo/PatchText",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_YACCHAUYO_TEXT_SCHEMA: ::grpcio::Method<super::yacchauyo::TextsQuery, super::yacchauyo::Schema> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/Yacchauyo/TextSchema",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_YACCHAUYO_PATCH_SCHEMA: ::grpcio::Method<super::yacchauyo::Schema, super::yacchauyo::Schema> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/Yacchauyo/PatchSchema",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_YACCHAUYO_QUERY_FRAGMENTS: ::grpcio::Method<super::yacchauyo::FragmentsQuery, super::yacchauyo::FragmentsQuery> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/Yacchauyo/QueryFragments",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

pub struct YacchauyoClient {
    client: ::grpcio::Client,
}

impl YacchauyoClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        YacchauyoClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn texts_index_opt(&self, req: &super::yacchauyo::TextsQuery, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::yacchauyo::Texts> {
        self.client.unary_call(&METHOD_YACCHAUYO_TEXTS_INDEX, req, opt)
    }

    pub fn texts_index(&self, req: &super::yacchauyo::TextsQuery) -> ::grpcio::Result<super::yacchauyo::Texts> {
        self.texts_index_opt(req, ::grpcio::CallOption::default())
    }

    pub fn texts_index_async_opt(&self, req: &super::yacchauyo::TextsQuery, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::yacchauyo::Texts>> {
        self.client.unary_call_async(&METHOD_YACCHAUYO_TEXTS_INDEX, req, opt)
    }

    pub fn texts_index_async(&self, req: &super::yacchauyo::TextsQuery) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::yacchauyo::Texts>> {
        self.texts_index_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_text_opt(&self, req: &super::yacchauyo::Text, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::yacchauyo::Text> {
        self.client.unary_call(&METHOD_YACCHAUYO_CREATE_TEXT, req, opt)
    }

    pub fn create_text(&self, req: &super::yacchauyo::Text) -> ::grpcio::Result<super::yacchauyo::Text> {
        self.create_text_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_text_async_opt(&self, req: &super::yacchauyo::Text, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::yacchauyo::Text>> {
        self.client.unary_call_async(&METHOD_YACCHAUYO_CREATE_TEXT, req, opt)
    }

    pub fn create_text_async(&self, req: &super::yacchauyo::Text) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::yacchauyo::Text>> {
        self.create_text_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_text_opt(&self, req: &super::yacchauyo::Text, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::yacchauyo::Text> {
        self.client.unary_call(&METHOD_YACCHAUYO_PATCH_TEXT, req, opt)
    }

    pub fn patch_text(&self, req: &super::yacchauyo::Text) -> ::grpcio::Result<super::yacchauyo::Text> {
        self.patch_text_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_text_async_opt(&self, req: &super::yacchauyo::Text, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::yacchauyo::Text>> {
        self.client.unary_call_async(&METHOD_YACCHAUYO_PATCH_TEXT, req, opt)
    }

    pub fn patch_text_async(&self, req: &super::yacchauyo::Text) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::yacchauyo::Text>> {
        self.patch_text_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn text_schema_opt(&self, req: &super::yacchauyo::TextsQuery, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::yacchauyo::Schema> {
        self.client.unary_call(&METHOD_YACCHAUYO_TEXT_SCHEMA, req, opt)
    }

    pub fn text_schema(&self, req: &super::yacchauyo::TextsQuery) -> ::grpcio::Result<super::yacchauyo::Schema> {
        self.text_schema_opt(req, ::grpcio::CallOption::default())
    }

    pub fn text_schema_async_opt(&self, req: &super::yacchauyo::TextsQuery, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::yacchauyo::Schema>> {
        self.client.unary_call_async(&METHOD_YACCHAUYO_TEXT_SCHEMA, req, opt)
    }

    pub fn text_schema_async(&self, req: &super::yacchauyo::TextsQuery) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::yacchauyo::Schema>> {
        self.text_schema_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_schema_opt(&self, req: &super::yacchauyo::Schema, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::yacchauyo::Schema> {
        self.client.unary_call(&METHOD_YACCHAUYO_PATCH_SCHEMA, req, opt)
    }

    pub fn patch_schema(&self, req: &super::yacchauyo::Schema) -> ::grpcio::Result<super::yacchauyo::Schema> {
        self.patch_schema_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_schema_async_opt(&self, req: &super::yacchauyo::Schema, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::yacchauyo::Schema>> {
        self.client.unary_call_async(&METHOD_YACCHAUYO_PATCH_SCHEMA, req, opt)
    }

    pub fn patch_schema_async(&self, req: &super::yacchauyo::Schema) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::yacchauyo::Schema>> {
        self.patch_schema_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn query_fragments_opt(&self, req: &super::yacchauyo::FragmentsQuery, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::yacchauyo::FragmentsQuery> {
        self.client.unary_call(&METHOD_YACCHAUYO_QUERY_FRAGMENTS, req, opt)
    }

    pub fn query_fragments(&self, req: &super::yacchauyo::FragmentsQuery) -> ::grpcio::Result<super::yacchauyo::FragmentsQuery> {
        self.query_fragments_opt(req, ::grpcio::CallOption::default())
    }

    pub fn query_fragments_async_opt(&self, req: &super::yacchauyo::FragmentsQuery, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::yacchauyo::FragmentsQuery>> {
        self.client.unary_call_async(&METHOD_YACCHAUYO_QUERY_FRAGMENTS, req, opt)
    }

    pub fn query_fragments_async(&self, req: &super::yacchauyo::FragmentsQuery) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::yacchauyo::FragmentsQuery>> {
        self.query_fragments_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Yacchauyo {
    fn texts_index(&self, ctx: ::grpcio::RpcContext, req: super::yacchauyo::TextsQuery, sink: ::grpcio::UnarySink<super::yacchauyo::Texts>);
    fn create_text(&self, ctx: ::grpcio::RpcContext, req: super::yacchauyo::Text, sink: ::grpcio::UnarySink<super::yacchauyo::Text>);
    fn patch_text(&self, ctx: ::grpcio::RpcContext, req: super::yacchauyo::Text, sink: ::grpcio::UnarySink<super::yacchauyo::Text>);
    fn text_schema(&self, ctx: ::grpcio::RpcContext, req: super::yacchauyo::TextsQuery, sink: ::grpcio::UnarySink<super::yacchauyo::Schema>);
    fn patch_schema(&self, ctx: ::grpcio::RpcContext, req: super::yacchauyo::Schema, sink: ::grpcio::UnarySink<super::yacchauyo::Schema>);
    fn query_fragments(&self, ctx: ::grpcio::RpcContext, req: super::yacchauyo::FragmentsQuery, sink: ::grpcio::UnarySink<super::yacchauyo::FragmentsQuery>);
}

pub fn create_yacchauyo<S: Yacchauyo + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_YACCHAUYO_TEXTS_INDEX, move |ctx, req, resp| {
        instance.texts_index(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_YACCHAUYO_CREATE_TEXT, move |ctx, req, resp| {
        instance.create_text(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_YACCHAUYO_PATCH_TEXT, move |ctx, req, resp| {
        instance.patch_text(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_YACCHAUYO_TEXT_SCHEMA, move |ctx, req, resp| {
        instance.text_schema(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_YACCHAUYO_PATCH_SCHEMA, move |ctx, req, resp| {
        instance.patch_schema(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_YACCHAUYO_QUERY_FRAGMENTS, move |ctx, req, resp| {
        instance.query_fragments(ctx, req, resp)
    });
    builder.build()
}
