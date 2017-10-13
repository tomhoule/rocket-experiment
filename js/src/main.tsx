/// <reference path='./global.d.ts' />
import { render } from 'react-dom'
import * as React from 'react'
import { reducerWithInitialState } from 'typescript-fsa-reducers'
import * as redux from 'redux'
import { createEpicMiddleware } from 'redux-observable'
import { Provider } from 'react-redux'
import App from './App'
import * as a from './actions'
import { rootEpic } from './epics'
import * as epicUtils from 'epic-utils'

const reducer = reducerWithInitialState({ status: '还可以' })
    .case(a.changeStatus, (state, { newStatus }) => ({ status: newStatus }))

function main() {
    const epicMiddleware = createEpicMiddleware(rootEpic, {
        dependencies: {
            get: epicUtils.get,
        },
    })
    const store = redux.createStore(reducer, redux.applyMiddleware(epicMiddleware))
    const root = document.getElementById('react-root')
    render(<Provider store={store}><App /></Provider>, root)
}

main()
