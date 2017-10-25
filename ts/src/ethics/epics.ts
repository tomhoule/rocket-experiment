import { ActionsObservable, combineEpics, Epic } from 'redux-observable'
import * as actions from './actions'
import * as Rx from 'rxjs/Rx'
import { AsyncActionCreators } from 'typescript-fsa'
import 'typescript-fsa-redux-observable'
import { InjectedDependencies } from 'types'
import { AppState } from './reducers'
import * as api from 'api-types'
import { push } from 'react-router-redux'

type AppEpic = Epic<Action, AppState, InjectedDependencies>

const schemaEpic: AppEpic= (action$, store, d) => {
  const filtered$ = action$.filter(() => !store.getState().schema)
  return d.get(filtered$, actions.getSchema, '/v1/ethics/schema')
}

const editions: AppEpic = (action$, store, d) => {
  const filtered$ = action$.filter(() => !store.getState().editions.index.length)
  return d.get(filtered$, actions.getEditions, '/v1/ethics/editions')
}

const createEdition: AppEpic = (action$, store, d) =>
  action$
    .ofAction(actions.create.started)
    .throttle(() => Rx.Observable.merge(
      action$.ofAction(actions.create.done).mapTo(null),
      action$.ofAction(actions.create.failed)).mapTo(null))
    .flatMap(({ payload }) => Rx.Observable.fromPromise(
      d.post(actions.create, payload, 'http://localhost:8008/v1/ethics/editions')))

const createEditionSuccess: AppEpic = (action$, store, d) =>
  action$
    .ofAction(actions.create.done)
    .flatMap(() => [push('/ethics/editions')])

const getFragments: AppEpic = (action$, store, d) =>
  action$
    .ofAction(actions.getFragments.started)
    .distinctUntilChanged((a1, a2) => a1.payload.slug === a2.payload.slug)
    .flatMap(({ payload }) => {
      const editions = store.getState().editions.index
      const extractEdition = (ed: api.RepositoryEdition[]): api.RepositoryEdition[] =>
        ed.filter(elem => elem.slug === payload.slug)
      return Rx.Observable
        .from(extractEdition(editions))
        .merge(
          action$
            .ofAction(actions.getEditions.done)
            .flatMap((action) => extractEdition(action.payload.result.data || [])))
        .take(1)
        .map((edition): [{ slug: string }, string] => [payload, edition.id as string])
    })
    .flatMap(async ([params, editionId]) =>
      await d.simpleGet(`/v1/ethics/editions/${editionId}/fragments`)
        .then(result => actions.getFragments.done({ params, result }))
        .catch(error => actions.getFragments.failed({ params, error })))

export const rootEpic = combineEpics(
  schemaEpic,
  editions,
  createEdition,
  createEditionSuccess,
  getFragments,
)
