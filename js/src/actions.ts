import actionCreatorFactory from 'typescript-fsa'

const actionCreator = actionCreatorFactory('main')
export const changeStatus = actionCreator<{ newStatus: string }>('CHANGE_STATUS')

export const getSchema = actionCreator.async<
    {},
    Schema,
    any
>('GET_SCHEMA')
