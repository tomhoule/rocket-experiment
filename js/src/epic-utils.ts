import { ActionsObservable, combineEpics, Epic } from 'redux-observable'
import 'typescript-fsa-redux-observable'
import * as actions from './actions'
import * as Rx from 'rxjs'
// import * as api from './api'
import { AsyncActionCreators } from 'typescript-fsa'
import * as jspb from 'google-protobuf'
import { grpc, Code, Metadata, BrowserHeaders } from 'grpc-web-client'
import { EthicsRepository } from './rpc/repository_pb_service'

export function get<I, S, F>(
    obs$: ActionsObservable<Action>,
    ty: AsyncActionCreators<I, S, F>,
    url: string,
    params?: { [key: string]: string }
): Rx.Observable<Action> {
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

export function rpc<
    Req extends jspb.Message,
    Res extends jspb.Message
>(
    method: any,
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
