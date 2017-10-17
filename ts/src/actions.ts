import actionCreatorFactory from 'typescript-fsa'
import * as api from './rpc/repository_pb'

const actionCreator = actionCreatorFactory('main')
export const changeStatus = actionCreator<{ newStatus: string }>('CHANGE_STATUS')

export const getSchema = actionCreator.async<
    api.GetSchemaParams.AsObject,
    api.EthicsSchema.AsObject,
    never
>('GET_SCHEMA')

export const editionMergeChanges = actionCreator<Partial<api.Edition.AsObject>>(
    'EDITION_MERGE_CHANGES'
)
export const editionSetChanges = actionCreator<Partial<api.Edition.AsObject>>(
    'EDITION_SET_CHANGES'
)

export const getEditions = actionCreator.async<
    api.GetEditionsParams.AsObject,
    api.Editions.AsObject,
    never
>('EDITIONS')
