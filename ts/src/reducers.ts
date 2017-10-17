import { reducerWithInitialState } from 'typescript-fsa-reducers'
import * as a from './actions'
import * as api from 'rpc/repository_pb'
import * as redux from 'redux'

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
    editions: CrudState<api.Edition.AsObject>
}

export type SchemaReducerState = api.EthicsSchema.AsObject | null

const statusReducer = reducerWithInitialState('还可以')
    .case(a.changeStatus, (state, { newStatus }) => newStatus)

const schemaReducer = reducerWithInitialState(null as SchemaReducerState)
    .case(a.getSchema.done, (state, { result }) => result)

const editionsReducer = reducerWithInitialState(crudState<api.Edition.AsObject>())
    .case(a.getEditions.done, (state, changes) => ({ ...state, index: changes.result.dataList }))

export const reducers = redux.combineReducers<AppState>({
    status: statusReducer,
    schema: schemaReducer,
    editions: editionsReducer,
})
