import actionCreatorFactory from 'typescript-fsa'
import * as api from 'typescript-fetch-api/api'

const actionCreator = actionCreatorFactory('main')
export const changeStatus = actionCreator<{ newStatus: string }>('CHANGE_STATUS')

const client = new api.EthicsRepositoryApi()
client.getSchema()

export const getSchema = actionCreator.async<
    {},
    api.RepositoryEthicsSchema,
    never
>('GET_SCHEMA')

export const editionMergeChanges = actionCreator<Partial<api.RepositoryEdition>>(
    'EDITION_MERGE_CHANGES'
)
export const editionSetChanges = actionCreator<Partial<api.RepositoryEdition>>(
    'EDITION_SET_CHANGES'
)

export const getEditions = actionCreator.async<
    {},
    api.RepositoryEditions,
    never
>('EDITIONS')
