import actionCreatorFactory from 'typescript-fsa'
import * as api from 'api-types'
import { GrpcStatus } from './types'

const actionCreator = actionCreatorFactory('main')
export const changeStatus = actionCreator<{ newStatus: string }>('CHANGE_STATUS')

export const create = actionCreator.async<
  Partial<api.RepositoryEdition>,
  api.RepositoryEdition,
  GrpcStatus
>('CREATE')

export const getSchema = actionCreator.async<
    {},
    api.RepositoryEthicsSchema,
    any
>('GET_SCHEMA')

export const mergeChanges = actionCreator<Partial<api.RepositoryEdition>>(
    'EDITION_MERGE_CHANGES'
)
export const setChanges = actionCreator<Partial<api.RepositoryEdition>>(
    'EDITION_SET_CHANGES'
)

export const getEditions = actionCreator.async<
    {},
    api.RepositoryEditions,
    never
>('EDITIONS')
