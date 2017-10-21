import { ActionsObservable, combineEpics, Epic } from 'redux-observable'
import 'typescript-fsa-redux-observable'
import * as actions from './actions'
import * as Rx from 'rxjs'
import { AsyncActionCreators, Success, ActionCreator, Action } from 'typescript-fsa'
import * as jspb from 'google-protobuf'

export function get<I, S, F>(
  obs$: ActionsObservable<{ type: string }>,
  ty: AsyncActionCreators<I, S, F>,
  url: string,
  params?: { [key: string]: string }
): Rx.Observable<Action<any>> {
  return obs$
    .ofAction(ty.started)
    .mergeMap(async action =>
      await fetch(`http://localhost:8008${url}`)
        .then(async r => [action.payload, await r.json()])
        .catch(err => [action.payload, err]))
    .do(res => console.log(res))
    .mergeMap(([params, result]) => [ty.done({ params, result })])
    .catch(([params, error]) => [ty.failed({ params , error })])
}
