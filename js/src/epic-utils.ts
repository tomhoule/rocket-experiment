import { ActionsObservable, combineEpics, Epic } from 'redux-observable'
import 'typescript-fsa-redux-observable'
import * as actions from './actions'
import * as Rx from 'rxjs'
import * as api from './api'
import { AsyncActionCreators } from 'typescript-fsa'
import * as jspb from 'google-protobuf'
import { grpc, Code, Metadata } from 'grpc-web-client'

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

// interface RPCAction<MSG, MTD> {
//     message: MSG,
//     method: MTD,
// }

// export function rpc<MSG extends jspb.Message, MTD, S, F, M>(
//     obs$: ActionsObservable<Action>,
//     ty: AsyncActionCreators<RPCAction<MSG, MTD>, S, F>,
// ): Rx.Observable<Action> {
//     return obs$
//         .ofAction(ty.started)
//         .mergeMap(async (action) => {
//             const req = new Promise(resolve =>
//                 grpc.invoke(action.payload.message, {
//                     request: action.payload.method
//                     host:
//                 })
//         })
//     const req = new Promise(resolve => grpc.invoke(method
// }
