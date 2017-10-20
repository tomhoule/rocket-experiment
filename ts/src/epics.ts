import { ActionsObservable, combineEpics, Epic } from 'redux-observable'
import * as actions from './actions'
import * as Rx from 'rxjs/Rx'
import { AsyncActionCreators } from 'typescript-fsa'
import 'typescript-fsa-redux-observable'
import { InjectedDependencies } from './types'
import { AppState } from './reducers'
import * as api from 'typescript-fetch-api/api'

type AppEpic = Epic<Action, AppState, InjectedDependencies>

  const schemaEpic: AppEpic= (action$, store, d) =>
  action$
    .ofAction(actions.getSchema.started)
    .mergeMap(async ({ payload: params }) =>
      await d.client.getSchema(params)
      .then(result => actions.getSchema.done({ params, result })))

const editions: AppEpic = (action$, store, d) =>
  action$
    .ofAction(actions.getEditions.started)
    .mergeMap(async ({ payload: params }) => {
      const result = await d.client.getEditions(params)
      return actions.getEditions.done({ params, result })
    })

export const rootEpic = combineEpics() // schemaEpic, editions)
