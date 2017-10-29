interface Action {
    type: string
}

interface Edition {
  id: string
  title: string
  editior: string
  year: string
  language_code: string
  slug: string
}

interface Fragment {
  id: string
  edition_id: string
  fragment_path: string
  value: string
}

type NodeType = string | { 'Scope': string }

interface SchemaNode {
  node_type: NodeType
  num: number | null
  children: SchemaNode[]
}

type EthicsSchema = SchemaNode

declare module '*.scss' {
  const styles: any
  export = styles
}

type Errors = Record<string, string>

interface GetPayload extends Record<string, any> {
  params?: Record<string, string>
}

interface CrudState<T> {
  index: T[]
  single: T | null
  changes: Partial<T>
  errors: Errors
}

interface AppState {
    schema: SchemaReducerState
    editions: CrudState<Edition>
}

interface JApiSuccessResponse {
  data: any
  errors?: any[]
  meta?: any
}

interface JApiErrorResponse {
  data?: any
  errors: any[]
  meta?: any
}

interface JApiResponse = JApiSuccessResponse | JApiErrorResponse

type SchemaReducerState = EthicsSchema | null
