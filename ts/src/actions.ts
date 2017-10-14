import actionCreatorFactory from 'typescript-fsa'
import * as api from './rpc/repository_pb'

const actionCreator = actionCreatorFactory('main')
export const changeStatus = actionCreator<{ newStatus: string }>('CHANGE_STATUS')

export const getSchema = actionCreator.async<
    {},
    api.EthicsSchema.AsObject,
    never
>('GET_SCHEMA')

export const patchEdition = actionCreator.async<
    api.EditionPatch.AsObject,
    api.Edition.AsObject,
    never
>('PATCH_EDITION')
