import * as epicUtils from 'epic-utils'
import * as api from 'api-types'

export interface InjectedDependencies {
  get: typeof epicUtils.get
  post: typeof epicUtils.post
}

export enum GrpcStatusCode {
  Ok = 0,
  Cancelled = 1,
  Unknown = 2,
  InvalidArgument = 3,
  DeadlineExceeded = 4,
  NotFound = 5,
  AlreadyExists = 6,
  PermissionDenied = 7,
  Unauthenticated = 16,
  ResourceExhausted = 8,
  FailedPrecondition = 9,
  Aborted = 10,
  OutOfRange = 11,
  Unimplemented = 12,
  Internal = 13,
  Unavailable = 14,
  DataLoss = 15,
}

export interface GrpcStatus {
  code: GrpcStatusCode
  error: string
}
