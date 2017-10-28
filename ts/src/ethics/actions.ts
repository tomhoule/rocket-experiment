import actionCreatorFactory from 'typescript-fsa'

const actionCreator = actionCreatorFactory('ethics')

export const editFragment = actionCreator.async<
  Fragment,
  Fragment,
  Errors
>('EDIT_FRAGMENT')

export const getFragments = actionCreator.async<
  { slug: string },
  Fragment[],
  Errors
>('GET_FRAGMENTS')

export const create = actionCreator.async<
  Partial<Edition>,
  Edition,
  Errors
>('CREATE')

export const getSchema = actionCreator.async<
  {},
  EthicsSchema,
  Errors
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
  {}
>('EDITIONS')

