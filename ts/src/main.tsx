/// <reference path='./global.d.ts' />
import { render } from 'react-dom'
import { BrowserRouter, Route } from 'react-router-dom'
import * as React from 'react'
import { reducerWithInitialState } from 'typescript-fsa-reducers'
import * as redux from 'redux'
import { createEpicMiddleware, Options } from 'redux-observable'
import { Provider } from 'react-redux'
import App from './App'
import * as a from './actions'
import { rootEpic } from './epics'
import * as epicUtils from 'epic-utils'
import 'rxjs'
import { reducers } from './reducers'
import * as api from 'typescript-fetch-api/api'
import { InjectedDependencies } from './types'

const composeEnhancers =
    (window as any).__REDUX_DEVTOOLS_EXTENSION_COMPOSE__
    ? (window as any).__REDUX_DEVTOOLS_EXTENSION_COMPOSE__
    : redux.compose

function main() {
  const epicMiddleware = createEpicMiddleware<Action, {}, InjectedDependencies>(rootEpic, {
    dependencies: {
      client: new api.EthicsRepositoryApi(),
    },
  })
  const store = redux.createStore(reducers, composeEnhancers(redux.applyMiddleware(epicMiddleware)))
  const root = document.getElementById('react-root')
  render(
    <BrowserRouter>
      <Provider store={store}>
        <Route path='/' component={App} />
      </Provider>
    </BrowserRouter>,
    root)
}

main()
