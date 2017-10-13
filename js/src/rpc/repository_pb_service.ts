// package: 
// file: repository.proto

import * as repository_pb from "./repository_pb";
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
}
