import { reducerWithInitialState } from 'typescript-fsa-reducers'
import * as a from './actions'
import * as redux from 'redux'

interface CrudState<T> {
  index: T[]
  single: T | null
  changes: Partial<T>
  errors: Errors
}

function crudState<T>(): CrudState<T> {
  return {
    index: [],
    single: null,
    changes: {},
    errors: {},
  }
}

export interface AppState {
    schema: SchemaReducerState
    editions: CrudState<Edition>
}

export type SchemaReducerState = EthicsSchema | null

export const schemaReducer = reducerWithInitialState(null as SchemaReducerState)
    .case(a.getSchema.done, (state, { result }) => result)

export const editionsReducer = reducerWithInitialState(crudState<Edition>())
    .case(a.getEditions.done, (state, changes) => ({
      ...state,
      index: changes.result,
    }))
    .case(a.mergeChanges, (state, changes) => ({
      ...state,
      changes: { ...state.changes, ...changes },
    }))
    .case(a.setChanges, (state, changes) => ({ ...state, changes }))
    .case(a.create.failed, (state, errors) => ({
      ...state,
      errors: errors.error,
    }))
