import { reducerWithInitialState } from 'typescript-fsa-reducers'
import * as a from './actions'
import * as redux from 'redux'

function crudState<T>(): CrudState<T> {
  return {
    index: [],
    single: null,
    changes: {},
    errors: {},
  }
}

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
