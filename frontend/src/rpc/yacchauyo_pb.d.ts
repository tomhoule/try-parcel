// package: 
// file: yacchauyo.proto

import * as jspb from "google-protobuf";

export class Text extends jspb.Message {
  getId(): string;
  setId(value: string): void;

  getTitle(): string;
  setTitle(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Text.AsObject;
  static toObject(includeInstance: boolean, msg: Text): Text.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Text, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Text;
  static deserializeBinaryFromReader(message: Text, reader: jspb.BinaryReader): Text;
}

export namespace Text {
  export type AsObject = {
    id: string,
    title: string,
  }
}

export class Texts extends jspb.Message {
  clearTextsList(): void;
  getTextsList(): Array<Text>;
  setTextsList(value: Array<Text>): void;
  addTexts(value?: Text, index?: number): Text;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Texts.AsObject;
  static toObject(includeInstance: boolean, msg: Texts): Texts.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Texts, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Texts;
  static deserializeBinaryFromReader(message: Texts, reader: jspb.BinaryReader): Texts;
}

export namespace Texts {
  export type AsObject = {
    textsList: Array<Text.AsObject>,
  }
}

export class TextsQuery extends jspb.Message {
  getTitle(): string;
  setTitle(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): TextsQuery.AsObject;
  static toObject(includeInstance: boolean, msg: TextsQuery): TextsQuery.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: TextsQuery, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): TextsQuery;
  static deserializeBinaryFromReader(message: TextsQuery, reader: jspb.BinaryReader): TextsQuery;
}

export namespace TextsQuery {
  export type AsObject = {
    title: string,
  }
}

