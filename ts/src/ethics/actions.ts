import actionCreatorFactory from 'typescript-fsa'

const actionCreator = actionCreatorFactory('ethics')

export const getFragments = actionCreator.async<
  // api.getFragmentsParams,
  { slug: string },
  Fragment[],
  any
>('GET_FRAGMENTS')

export const create = actionCreator.async<
  Partial<Edition>,
  Edition,
  Errors
>('CREATE')

export const getSchema = actionCreator.async<
    {},
    EthicsSchema,
    any
>('GET_SCHEMA')

export const mergeChanges = actionCreator<Partial<Edition>>(
    'EDITION_MERGE_CHANGES'
)
export const setChanges = actionCreator<Partial<Edition>>(
    'EDITION_SET_CHANGES'
)

export const getEditions = actionCreator.async<
    {},
    Edition[],
    never
>('EDITIONS')

