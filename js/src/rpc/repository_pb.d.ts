// package: 
// file: repository.proto

import * as jspb from "google-protobuf";

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

