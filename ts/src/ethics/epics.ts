import { ActionsObservable, combineEpics, Epic } from 'redux-observable'
import * as actions from './actions'
import * as Rx from 'rxjs/Rx'
import { AsyncActionCreators } from 'typescript-fsa'
import 'typescript-fsa-redux-observable'
import { InjectedDependencies } from 'types'
import * as api from 'api-types'
import { push } from 'react-router-redux'

type AppEpic = Epic<Action, AppState, InjectedDependencies>

const schemaEpic: AppEpic= (action$, store, d) =>
  action$
    .ofAction(actions.getSchema.started)
    .filter(() => !store.getState().schema)
    .flatMap(({ payload }) => d.get('/v1/ethics/schema', payload))
    .map(result => actions.getSchema.done(result))
    .catch(err => Rx.Observable.of(actions.getSchema.failed(err)))

const editions: AppEpic = (action$, store, d) =>
  action$
    .ofAction(actions.getEditions.started)
    .distinctUntilChanged((a, b) => true) // only ever fetch once
    .flatMap(({ payload }) => d.get('/v1/ethics/editions', payload))
    .map(result => actions.getEditions.done(result))
    .catch(err => Rx.Observable.of(actions.getEditions.failed(err)))

const createEdition: AppEpic = (action$, store, d) =>
  action$
    .ofAction(actions.create.started)
    .throttle(() => Rx.Observable.merge(
      action$.ofAction(actions.create.done).mapTo(null),
      action$.ofAction(actions.create.failed)).mapTo(null))
    .flatMap(({ payload }) => Rx.Observable.fromPromise(
      d.post(actions.create, payload, '/v1/ethics/editions')))

const editFragment: AppEpic = (action$, store, d) =>
  action$
    .ofAction(actions.editFragment.started)
    .flatMap(({ payload }) =>
      d.put(`/v1/ethics/editions/${payload.edition_id}/fragments`, payload))
    .map(result => actions.editFragment.done(result))
    .catch(err => Rx.Observable.of(actions.editFragment.failed(err)))

const createEditionSuccess: AppEpic = (action$, store, d) =>
  action$
    .ofAction(actions.create.done)
    .flatMap(() => [push('/ethics/editions')])

const getFragments: AppEpic = (action$, store, d) =>
  action$
    .ofAction(actions.getFragments.started)
    .takeUntil(action$.ofAction(actions.getFragments.started))
    .flatMap(({ payload }) => {
      const editions = store.getState().editions.index
      const extractEdition = (ed: Edition[]): Edition[] =>
        ed.filter(elem => elem.slug === payload.slug)
      return Rx.Observable
        .from(extractEdition(editions))
        .merge(
          action$
            .ofAction(actions.getEditions.done)
            .flatMap((action) => extractEdition(action.payload.result)))
        .take(1)
        .map((edition): [{ slug: string }, string] => [payload, edition.id as string])
    })
    .flatMap(([params, editionId]) =>
      d.get(`/v1/ethics/editions/${editionId}/fragments`, params))
    .map(result => actions.getFragments.done(result))
    .catch(err => Rx.Observable.of(actions.getFragments.failed(err)))

export const rootEpic = combineEpics(
  schemaEpic,
  editions,
  createEdition,
  createEditionSuccess,
  getFragments,
)
