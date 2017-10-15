// package: 
// file: repository.proto

import * as repository_pb from "./repository_pb";
import * as google_protobuf_field_mask_pb from "google-protobuf/google/protobuf/field_mask_pb";
import * as google_protobuf_wrappers_pb from "google-protobuf/google/protobuf/wrappers_pb";
export class EthicsRepository {
  static serviceName = "EthicsRepository";
}
export namespace EthicsRepository {
  export class GetSchema {
    static readonly methodName = "GetSchema";
    static readonly service = EthicsRepository;
    static readonly requestStream = false;
    static readonly responseStream = false;
    static readonly requestType = repository_pb.GetSchemaParams;
    static readonly responseType = repository_pb.EthicsSchema;
  }
  export class GetEditions {
    static readonly methodName = "GetEditions";
    static readonly service = EthicsRepository;
    static readonly requestStream = false;
    static readonly responseStream = false;
    static readonly requestType = repository_pb.GetEditionsParams;
    static readonly responseType = repository_pb.Editions;
  }
  export class CreateEdition {
    static readonly methodName = "CreateEdition";
    static readonly service = EthicsRepository;
    static readonly requestStream = false;
    static readonly responseStream = false;
    static readonly requestType = repository_pb.Edition;
    static readonly responseType = repository_pb.Edition;
  }
  export class PatchEdition {
    static readonly methodName = "PatchEdition";
    static readonly service = EthicsRepository;
    static readonly requestStream = false;
    static readonly responseStream = false;
    static readonly requestType = repository_pb.EditionPatch;
    static readonly responseType = repository_pb.Edition;
  }
}
