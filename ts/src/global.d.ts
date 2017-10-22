interface Action {
    type: string
}

declare module 'core-js/library/fn/object/assign';

declare module '*.scss' {
  const styles: any
  export = styles
}

type Dictionary<T> = { [key: string]: T }
type Errors = Dictionary<string>
