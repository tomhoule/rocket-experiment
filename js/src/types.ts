import * as epicUtils from 'epic-utils'

export interface InjectedDependencies {
    get: typeof epicUtils.get
}
