import { ActionsObservable, combineEpics, Epic } from 'redux-observable'
import 'typescript-fsa-redux-observable'
import * as Rx from 'rxjs'
import { Action } from 'redux'
import { AsyncActionCreators, Success, ActionCreator } from 'typescript-fsa'
import * as jspb from 'google-protobuf'
import { GrpcStatus } from './types'

export function get<I, S, F>(
  obs$: Rx.Observable<Action>,
  ty: AsyncActionCreators<I, S, F>,
  url: string,
  params?: { [key: string]: string }
): Rx.Observable<any> {
  return (obs$ as ActionsObservable<Action>)
    .ofAction(ty.started)
    .mergeMap(async action =>
      await fetch(`http://localhost:8008${url}`)
        .then(async r => [action.payload, await r.json()])
        .catch(err => [action.payload, err]))
    .mergeMap(([params, result]) => [ty.done({ params, result })])
    .catch(([params, error]) => [ty.failed({ params , error })])
}

export async function post<T, U>(
  actions: AsyncActionCreators<T, U, GrpcStatus>,
  params: T,
  url: string,
  rqinit: RequestInit = {}
): Promise<Action> {
  const response = await fetch(url, {
    ...rqinit,
    method: 'POST',
    body: JSON.stringify(params),
    headers: { 'Content-Type': 'application/json' },
  })
  const json = await response.json()
  if (response.status === 200) {
    return actions.done({ params, result: json })
  } else {
    return actions.failed({ params, error: json })
  }
}
