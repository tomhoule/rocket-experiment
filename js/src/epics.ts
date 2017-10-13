import { ActionsObservable, combineEpics, Epic } from 'redux-observable'
import 'typescript-fsa-redux-observable'
import * as actions from './actions'
import * as Rx from 'rxjs'
import * as api from './api'
import { AsyncActionCreators } from 'typescript-fsa'
import { InjectedDependencies } from './types'

const schemaEpic: Epic<Action, AppState, InjectedDependencies> = (action$, store, d) =>
    d.get(action$, actions.getSchema, '/schema')

export const rootEpic = combineEpics(schemaEpic)
