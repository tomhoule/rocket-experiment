// package: 
// file: repository.proto

import * as jspb from "google-protobuf";
import * as google_protobuf_timestamp_pb from "google-protobuf/google/protobuf/timestamp_pb";

export class GetEditionsParams extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetEditionsParams.AsObject;
  static toObject(includeInstance: boolean, msg: GetEditionsParams): GetEditionsParams.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetEditionsParams, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetEditionsParams;
  static deserializeBinaryFromReader(message: GetEditionsParams, reader: jspb.BinaryReader): GetEditionsParams;
}

export namespace GetEditionsParams {
  export type AsObject = {
  }
}

export class Edition extends jspb.Message {
  getTitle(): string;
  setTitle(value: string): void;

  getSlug(): string;
  setSlug(value: string): void;

  getEditor(): string;
  setEditor(value: string): void;

  getYear(): number;
  setYear(value: number): void;

  getLanguageCode(): string;
  setLanguageCode(value: string): void;

  getCreatedAt(): string;
  setCreatedAt(value: string): void;

  getUpdatedAt(): string;
  setUpdatedAt(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Edition.AsObject;
  static toObject(includeInstance: boolean, msg: Edition): Edition.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Edition, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Edition;
  static deserializeBinaryFromReader(message: Edition, reader: jspb.BinaryReader): Edition;
}

export namespace Edition {
  export type AsObject = {
    title: string,
    slug: string,
    editor: string,
    year: number,
    languageCode: string,
    createdAt: string,
    updatedAt: string,
  }
}

export class Editions extends jspb.Message {
  clearDataList(): void;
  getDataList(): Array<Edition>;
  setDataList(value: Array<Edition>): void;
  addData(value?: Edition, index?: number): Edition;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Editions.AsObject;
  static toObject(includeInstance: boolean, msg: Editions): Editions.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Editions, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Editions;
  static deserializeBinaryFromReader(message: Editions, reader: jspb.BinaryReader): Editions;
}

export namespace Editions {
  export type AsObject = {
    dataList: Array<Edition.AsObject>,
  }
}

export class GetSchemaParams extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetSchemaParams.AsObject;
  static toObject(includeInstance: boolean, msg: GetSchemaParams): GetSchemaParams.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetSchemaParams, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetSchemaParams;
  static deserializeBinaryFromReader(message: GetSchemaParams, reader: jspb.BinaryReader): GetSchemaParams;
}

export namespace GetSchemaParams {
  export type AsObject = {
  }
}

export class EthicsSchema extends jspb.Message {
  clearPartsList(): void;
  getPartsList(): Array<EthicsSchema.Node>;
  setPartsList(value: Array<EthicsSchema.Node>): void;
  addParts(value?: EthicsSchema.Node, index?: number): EthicsSchema.Node;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): EthicsSchema.AsObject;
  static toObject(includeInstance: boolean, msg: EthicsSchema): EthicsSchema.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: EthicsSchema, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): EthicsSchema;
  static deserializeBinaryFromReader(message: EthicsSchema, reader: jspb.BinaryReader): EthicsSchema;
}

export namespace EthicsSchema {
  export type AsObject = {
    partsList: Array<EthicsSchema.Node.AsObject>,
  }

  export class Node extends jspb.Message {
    getNodeType(): EthicsSchema.NodeType;
    setNodeType(value: EthicsSchema.NodeType): void;

    hasNum(): boolean;
    clearNum(): void;
    getNum(): number;
    setNum(value: number): void;

    hasTitle(): boolean;
    clearTitle(): void;
    getTitle(): string;
    setTitle(value: string): void;

    clearChildrenList(): void;
    getChildrenList(): Array<EthicsSchema.Node>;
    setChildrenList(value: Array<EthicsSchema.Node>): void;
    addChildren(value?: EthicsSchema.Node, index?: number): EthicsSchema.Node;

    getIdentifierCase(): Node.IdentifierCase;
    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Node.AsObject;
    static toObject(includeInstance: boolean, msg: Node): Node.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: Node, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Node;
    static deserializeBinaryFromReader(message: Node, reader: jspb.BinaryReader): Node;
  }

  export namespace Node {
    export type AsObject = {
      nodeType: EthicsSchema.NodeType,
      num: number,
      title: string,
      childrenList: Array<EthicsSchema.Node.AsObject>,
    }

    export enum IdentifierCase {
      IDENTIFIER_NOT_SET = 0,
      NUM = 2,
      TITLE = 3,
    }
  }

  export enum NodeType {
    UNSPECIFIED = 0,
    ANONYMOUS_FRAGMENT = 1,
    ALITER = 2,
    APPENDINX = 3,
    AXIOMA = 4,
    CAPUT = 5,
    COROLLARIUM = 6,
    DEFINITIO = 7,
    DEMONSTRATIO = 8,
    EXPLICATIO = 9,
    LEMMA = 11,
    PARS = 12,
    POSTULATUM = 13,
    PRAEFATIO = 14,
    PROPOSITIO = 15,
    SCHOLIUM = 16,
  }
}

