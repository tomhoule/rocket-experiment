import * as epicUtils from 'epic-utils'
import * as api from 'api-types'

export interface InjectedDependencies {
  get: typeof epicUtils.get
  post: typeof epicUtils.post
  put: typeof epicUtils.put
  simpleGet: typeof epicUtils.simpleGet
}
