// GENERATED CODE -- DO NOT EDIT!

'use strict';
var grpc = require('grpc');
var proto_yacchauyo_pb = require('../proto/yacchauyo_pb.js');
var google_protobuf_timestamp_pb = require('google-protobuf/google/protobuf/timestamp_pb.js');

function serialize_FragmentsQuery(arg) {
  if (!(arg instanceof proto_yacchauyo_pb.FragmentsQuery)) {
    throw new Error('Expected argument of type FragmentsQuery');
  }
  return new Buffer(arg.serializeBinary());
}

function deserialize_FragmentsQuery(buffer_arg) {
  return proto_yacchauyo_pb.FragmentsQuery.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_Schema(arg) {
  if (!(arg instanceof proto_yacchauyo_pb.Schema)) {
    throw new Error('Expected argument of type Schema');
  }
  return new Buffer(arg.serializeBinary());
}

function deserialize_Schema(buffer_arg) {
  return proto_yacchauyo_pb.Schema.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_Text(arg) {
  if (!(arg instanceof proto_yacchauyo_pb.Text)) {
    throw new Error('Expected argument of type Text');
  }
  return new Buffer(arg.serializeBinary());
}

function deserialize_Text(buffer_arg) {
  return proto_yacchauyo_pb.Text.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_Texts(arg) {
  if (!(arg instanceof proto_yacchauyo_pb.Texts)) {
    throw new Error('Expected argument of type Texts');
  }
  return new Buffer(arg.serializeBinary());
}

function deserialize_Texts(buffer_arg) {
  return proto_yacchauyo_pb.Texts.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_TextsQuery(arg) {
  if (!(arg instanceof proto_yacchauyo_pb.TextsQuery)) {
    throw new Error('Expected argument of type TextsQuery');
  }
  return new Buffer(arg.serializeBinary());
}

function deserialize_TextsQuery(buffer_arg) {
  return proto_yacchauyo_pb.TextsQuery.deserializeBinary(new Uint8Array(buffer_arg));
}


var YacchauyoService = exports.YacchauyoService = {
  textsIndex: {
    path: '/Yacchauyo/TextsIndex',
    requestStream: false,
    responseStream: false,
    requestType: proto_yacchauyo_pb.TextsQuery,
    responseType: proto_yacchauyo_pb.Texts,
    requestSerialize: serialize_TextsQuery,
    requestDeserialize: deserialize_TextsQuery,
    responseSerialize: serialize_Texts,
    responseDeserialize: deserialize_Texts,
  },
  createText: {
    path: '/Yacchauyo/CreateText',
    requestStream: false,
    responseStream: false,
    requestType: proto_yacchauyo_pb.Text,
    responseType: proto_yacchauyo_pb.Text,
    requestSerialize: serialize_Text,
    requestDeserialize: deserialize_Text,
    responseSerialize: serialize_Text,
    responseDeserialize: deserialize_Text,
  },
  patchText: {
    path: '/Yacchauyo/PatchText',
    requestStream: false,
    responseStream: false,
    requestType: proto_yacchauyo_pb.Text,
    responseType: proto_yacchauyo_pb.Text,
    requestSerialize: serialize_Text,
    requestDeserialize: deserialize_Text,
    responseSerialize: serialize_Text,
    responseDeserialize: deserialize_Text,
  },
  textSchema: {
    path: '/Yacchauyo/TextSchema',
    requestStream: false,
    responseStream: false,
    requestType: proto_yacchauyo_pb.TextsQuery,
    responseType: proto_yacchauyo_pb.Schema,
    requestSerialize: serialize_TextsQuery,
    requestDeserialize: deserialize_TextsQuery,
    responseSerialize: serialize_Schema,
    responseDeserialize: deserialize_Schema,
  },
  patchSchema: {
    path: '/Yacchauyo/PatchSchema',
    requestStream: false,
    responseStream: false,
    requestType: proto_yacchauyo_pb.Schema,
    responseType: proto_yacchauyo_pb.Schema,
    requestSerialize: serialize_Schema,
    requestDeserialize: deserialize_Schema,
    responseSerialize: serialize_Schema,
    responseDeserialize: deserialize_Schema,
  },
  queryFragments: {
    path: '/Yacchauyo/QueryFragments',
    requestStream: false,
    responseStream: false,
    requestType: proto_yacchauyo_pb.FragmentsQuery,
    responseType: proto_yacchauyo_pb.FragmentsQuery,
    requestSerialize: serialize_FragmentsQuery,
    requestDeserialize: deserialize_FragmentsQuery,
    responseSerialize: serialize_FragmentsQuery,
    responseDeserialize: deserialize_FragmentsQuery,
  },
};

exports.YacchauyoClient = grpc.makeGenericClientConstructor(YacchauyoService);
