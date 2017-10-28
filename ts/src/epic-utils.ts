import { API_URL } from 'config'
import { ActionsObservable, combineEpics, Epic } from 'redux-observable'
import 'typescript-fsa-redux-observable'
import * as Rx from 'rxjs'
import { Action } from 'redux'
import { AsyncActionCreators, Success, ActionCreator, Failure } from 'typescript-fsa'
import * as jspb from 'google-protobuf'

export async function post<T, U>(
  actions: AsyncActionCreators<T, U, Errors>,
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

async function extractBody(res: Response): Promise<any> {
  if (isJson(res)) {
    return res.json()
  } else {
    return res.text()
  }
}

function isJson(res: Response): boolean {
  return (res.headers.get('Content-Type') || '').includes('application/json')
}

export async function simpleGet<T extends GetPayload, U>(
  url: string,
  params?: T
): Promise<Success<T, U>> {
  const res = await fetch(`${API_URL}${url}`)
  if (res.status > 299) {
    if (isJson(res)) {
      const error = await res.json()
      throw { params, error }
    } else {
      const error = await res.text()
      throw { params, error }
    }
  }
  return extractBody(res)
}

export function get<T extends GetPayload, U>(
  url: string,
  payload?: T
): Rx.Observable<Success<T, U>> {
  return Rx.Observable.fromPromise(simpleGet(url, payload))
}
