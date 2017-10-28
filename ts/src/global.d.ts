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
  edition: string
  fragment_path: string
  value: string
}

interface SchemaNode {
  node_type: string
  num: number | null
  children: SchemaNode[]
}

type EthicsSchema = any

declare module '*.scss' {
  const styles: any
  export = styles
}

type Errors = Record<string, string>

interface GetPayload extends Record<string, any> {
  params?: Record<string, string>
}

