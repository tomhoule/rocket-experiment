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

async function extractError<T>(res: Response, params: T): Promise<never> {
    if (isJson(res)) {
      const error = await res.json()
      throw { params, error }
    } else {
      const error = await res.text()
      throw { params, error }
    }
}

function isJson(res: Response): boolean {
  return (res.headers.get('Content-Type') || '').includes('application/json')
}

export async function simpleGet<T extends GetPayload, U>(
  url: string,
  params: T
): Promise<Success<T, U>> {
  const res = await fetch(`${API_URL}${url}`)
  if (res.status > 299) {
    return extractError(res, params)
  }
  const result = await extractBody(res)
  return { params, result }
}

export function get<T extends GetPayload>(
  url: string,
  payload: T
): Rx.Observable<Success<T, any>> {
  return Rx.Observable.fromPromise(simpleGet(url, payload))
}

async function asyncPut<T>(url: string, params: T): Promise<Success<T, T>> {
  const res = await fetch(`${API_URL}${url}`, {
    method: 'PUT',
    body: JSON.stringify(params),
    headers: { 'Content-Type': 'application/json' },
  })
  if (res.status > 299) {
    return extractError(res, params)
  }
  const result = await extractBody(res)
  return { params, result }
}

export function put<T>(
  url: string,
  payload: T
): Rx.Observable<Success<T, T>> {
  return Rx.Observable.fromPromise(asyncPut(url, payload))
}
