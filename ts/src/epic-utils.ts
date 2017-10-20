import { ActionsObservable, combineEpics, Epic } from 'redux-observable'
import 'typescript-fsa-redux-observable'
import * as actions from './actions'
import * as Rx from 'rxjs'
import { AsyncActionCreators, Success, ActionCreator, Action } from 'typescript-fsa'
import * as jspb from 'google-protobuf'

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
