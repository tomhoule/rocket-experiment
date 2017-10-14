import { ActionsObservable, combineEpics, Epic } from 'redux-observable'
import 'typescript-fsa-redux-observable'
import * as actions from './actions'
import * as Rx from 'rxjs'
// import * as api from './api'
import { AsyncActionCreators, Success, ActionCreator, Action } from 'typescript-fsa'
import * as jspb from 'google-protobuf'
import { grpc, Code, Metadata, BrowserHeaders } from 'grpc-web-client'
import { EthicsRepository } from './rpc/repository_pb_service'

export function get<I, S, F>(
    obs$: ActionsObservable<Action<any>>,
    ty: AsyncActionCreators<I, S, F>,
    url: string,
    params?: { [key: string]: string }
): Rx.Observable<Action<any>> {
    return obs$
        .ofAction(ty.started)
        .mergeMap(async action => [action.payload, await fetch(url).then(r => r.json())])
        .mergeMap(([params, result]) => [ty.done({ params, result })])
}

interface MethodDefinition<Req, Res> {
    methodName: string
    service: any
    requestStream: boolean
    responseStream: boolean
    requestType: Req
    responseType: Res
}

export function rejoice<
    T,
    Result,
    U extends { toObject: () => Result }
>(action: ActionCreator<Success<T, Result>>, params: T): (result: U) => Action<Success<T, Result>> {
    return result => action({ params, result: result.toObject() })
}

export function rpc<
    Req extends jspb.Message = never,
    Res extends jspb.Message = never
>(
    method: grpc.MethodDefinition<Req, Res>,
    request: Req,
): Promise<Res> {
    return new Promise(resolve =>
        grpc.invoke(method, {
            request,
            host: 'https://localhost:8443',
            onMessage: (message: Res) => resolve(message),
            onEnd: (code: Code, message: string, trailers: BrowserHeaders) => console.log(code, message, trailers),
        }))
}
