import { reducerWithInitialState } from 'typescript-fsa-reducers'
import * as a from './actions'
import * as api from 'rpc/repository_pb'
import * as redux from 'redux'

export interface AppState {
    status: string
    schema: SchemaReducerState
}

export type SchemaReducerState = api.EthicsSchema.AsObject | null

const statusReducer = reducerWithInitialState('还可以')
    .case(a.changeStatus, (state, { newStatus }) => newStatus)

const schemaReducer = reducerWithInitialState(null as SchemaReducerState)
    .case(a.getSchema.done, (state, { result }) => result)

export const reducers = redux.combineReducers<AppState>({
    status: statusReducer,
    schema: schemaReducer,
})
