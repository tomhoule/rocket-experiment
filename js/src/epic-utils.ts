import { ActionsObservable, combineEpics, Epic } from 'redux-observable'
import 'typescript-fsa-redux-observable'
import * as actions from './actions'
import * as Rx from 'rxjs'
import * as api from './api'
import { AsyncActionCreators } from 'typescript-fsa'

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
