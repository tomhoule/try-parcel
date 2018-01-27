// package: 
// file: yacchauyo.proto

import * as yacchauyo_pb from "./yacchauyo_pb";
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
}
