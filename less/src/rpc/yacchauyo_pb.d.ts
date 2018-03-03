// package: 
// file: yacchauyo.proto

import * as jspb from "google-protobuf";
import * as google_protobuf_timestamp_pb from "google-protobuf/google/protobuf/timestamp_pb";

export class Text extends jspb.Message {
  getId(): string;
  setId(value: string): void;

  getTitle(): string;
  setTitle(value: string): void;

  getSlug(): string;
  setSlug(value: string): void;

  getAuthors(): string;
  setAuthors(value: string): void;

  getDescription(): string;
  setDescription(value: string): void;

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
    slug: string,
    authors: string,
    description: string,
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
  getId(): string;
  setId(value: string): void;

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
    id: string,
    title: string,
  }
}

export class Schema extends jspb.Message {
  getId(): string;
  setId(value: string): void;

  getTextId(): string;
  setTextId(value: string): void;

  clearPathsList(): void;
  getPathsList(): Array<string>;
  setPathsList(value: Array<string>): void;
  addPaths(value: string, index?: number): string;

  hasCreatedAt(): boolean;
  clearCreatedAt(): void;
  getCreatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setCreatedAt(value?: google_protobuf_timestamp_pb.Timestamp): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Schema.AsObject;
  static toObject(includeInstance: boolean, msg: Schema): Schema.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Schema, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Schema;
  static deserializeBinaryFromReader(message: Schema, reader: jspb.BinaryReader): Schema;
}

export namespace Schema {
  export type AsObject = {
    id: string,
    textId: string,
    pathsList: Array<string>,
    createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
  }
}

export class Fragment extends jspb.Message {
  getId(): string;
  setId(value: string): void;

  getSchemaPath(): string;
  setSchemaPath(value: string): void;

  getTextId(): string;
  setTextId(value: string): void;

  hasCreatedAt(): boolean;
  clearCreatedAt(): void;
  getCreatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setCreatedAt(value?: google_protobuf_timestamp_pb.Timestamp): void;

  getValue(): string;
  setValue(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Fragment.AsObject;
  static toObject(includeInstance: boolean, msg: Fragment): Fragment.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Fragment, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Fragment;
  static deserializeBinaryFromReader(message: Fragment, reader: jspb.BinaryReader): Fragment;
}

export namespace Fragment {
  export type AsObject = {
    id: string,
    schemaPath: string,
    textId: string,
    createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    value: string,
  }
}

export class FragmentsQuery extends jspb.Message {
  clearFragmentsList(): void;
  getFragmentsList(): Array<Fragment>;
  setFragmentsList(value: Array<Fragment>): void;
  addFragments(value?: Fragment, index?: number): Fragment;

  getPrefix(): string;
  setPrefix(value: string): void;

  getTextId(): string;
  setTextId(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): FragmentsQuery.AsObject;
  static toObject(includeInstance: boolean, msg: FragmentsQuery): FragmentsQuery.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: FragmentsQuery, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): FragmentsQuery;
  static deserializeBinaryFromReader(message: FragmentsQuery, reader: jspb.BinaryReader): FragmentsQuery;
}

export namespace FragmentsQuery {
  export type AsObject = {
    fragmentsList: Array<Fragment.AsObject>,
    prefix: string,
    textId: string,
  }
}

