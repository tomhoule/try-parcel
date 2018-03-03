// package: 
// file: yacchauyo.proto

import * as yacchauyo_pb from "./yacchauyo_pb";
import * as google_protobuf_timestamp_pb from "google-protobuf/google/protobuf/timestamp_pb";
export class Yacchauyo {
  static serviceName = "Yacchauyo";
}
export namespace Yacchauyo {
  export class TextsIndex {
    static readonly methodName = "TextsIndex";
    static readonly service = Yacchauyo;
    static readonly requestStream = false;
    static readonly responseStream = false;
    static readonly requestType = yacchauyo_pb.TextsQuery;
    static readonly responseType = yacchauyo_pb.Texts;
  }
  export class CreateText {
    static readonly methodName = "CreateText";
    static readonly service = Yacchauyo;
    static readonly requestStream = false;
    static readonly responseStream = false;
    static readonly requestType = yacchauyo_pb.Text;
    static readonly responseType = yacchauyo_pb.Text;
  }
  export class PatchText {
    static readonly methodName = "PatchText";
    static readonly service = Yacchauyo;
    static readonly requestStream = false;
    static readonly responseStream = false;
    static readonly requestType = yacchauyo_pb.Text;
    static readonly responseType = yacchauyo_pb.Text;
  }
  export class TextSchema {
    static readonly methodName = "TextSchema";
    static readonly service = Yacchauyo;
    static readonly requestStream = false;
    static readonly responseStream = false;
    static readonly requestType = yacchauyo_pb.TextsQuery;
    static readonly responseType = yacchauyo_pb.Schema;
  }
  export class PatchSchema {
    static readonly methodName = "PatchSchema";
    static readonly service = Yacchauyo;
    static readonly requestStream = false;
    static readonly responseStream = false;
    static readonly requestType = yacchauyo_pb.Schema;
    static readonly responseType = yacchauyo_pb.Schema;
  }
  export class QueryFragments {
    static readonly methodName = "QueryFragments";
    static readonly service = Yacchauyo;
    static readonly requestStream = false;
    static readonly responseStream = false;
    static readonly requestType = yacchauyo_pb.FragmentsQuery;
    static readonly responseType = yacchauyo_pb.FragmentsQuery;
  }
}
