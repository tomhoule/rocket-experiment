import { ActionsObservable, combineEpics, Epic } from 'redux-observable'
import * as actions from './actions'
import * as Rx from 'rxjs/Rx'
import { AsyncActionCreators } from 'typescript-fsa'
import 'typescript-fsa-redux-observable'
import { InjectedDependencies } from './types'
import { AppState } from './reducers'
import * as api from 'api-types'
import { get } from './epic-utils'

type AppEpic = Epic<Action, AppState, InjectedDependencies>

const schemaEpic: AppEpic= (action$, store, d) => {
  const filtered$ = action$.filter(() => !store.getState().schema)
  return get(filtered$, actions.getSchema, '/v1/ethics/schema')
}

const editions: AppEpic = (action$, store, d) => {
  const filtered$ = action$.filter(() => !store.getState().editions.index.length)
  return get(filtered$, actions.getEditions, '/v1/ethics/editions')
}

export const rootEpic = combineEpics(schemaEpic, editions)
