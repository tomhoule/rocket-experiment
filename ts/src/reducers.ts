import { reducerWithInitialState } from 'typescript-fsa-reducers'
import * as a from './actions'
import * as redux from 'redux'
import * as api from 'api-types'

interface CrudState<T> {
    index: T[]
    single: T | null
    changes: Partial<T>
}

function crudState<T>(): CrudState<T> {
    return {
        index: [],
        single: null,
        changes: {},
    }
}

export interface AppState {
    status: string
    schema: SchemaReducerState
    editions: CrudState<api.RepositoryEdition>
}

export type SchemaReducerState = api.RepositoryEthicsSchema | null

const statusReducer = reducerWithInitialState('还可以')
    .case(a.changeStatus, (state, { newStatus }) => newStatus)

const schemaReducer = reducerWithInitialState(null as SchemaReducerState)
    .case(a.getSchema.done, (state, { result }) => result)

const editionsReducer = reducerWithInitialState(crudState<api.RepositoryEdition>())
    .case(a.getEditions.done, (state, changes) => ({
      ...state,
      index: changes.result.data || [],
    }))
    .case(a.mergeChanges, (state, changes) => ({
      ...state,
      changes: { ...state.changes, ...changes },
    }))
    .case(a.setChanges, (state, changes) => ({ ...state, changes }))

export const reducers = redux.combineReducers<AppState>({
    status: statusReducer,
    schema: schemaReducer,
    editions: editionsReducer,
})
