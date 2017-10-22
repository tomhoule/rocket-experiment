/// <reference path='./global.d.ts' />
import 'babel-polyfill'
import { render } from 'react-dom'
import { BrowserRouter, Route } from 'react-router-dom'
import * as React from 'react'
import { reducerWithInitialState } from 'typescript-fsa-reducers'
import * as redux from 'redux'
import { createEpicMiddleware, Options } from 'redux-observable'
import { Provider } from 'react-redux'
import App from './App'
import { rootEpic as ethicsRootEpic } from './ethics/epics'
import * as epicUtils from 'epic-utils'
import 'rxjs'
import { ethicsReducers } from './ethics/reducers'
import * as api from 'api-types'
import { InjectedDependencies } from './types'

const composeEnhancers =
    (window as any).__REDUX_DEVTOOLS_EXTENSION_COMPOSE__
    ? (window as any).__REDUX_DEVTOOLS_EXTENSION_COMPOSE__
    : redux.compose

const rootEpic = ethicsRootEpic

function main() {
  const epicMiddleware = createEpicMiddleware<Action, {}, InjectedDependencies>(
    rootEpic,
    {
      dependencies: {
        get: epicUtils.get,
        post: epicUtils.post,
      },
    }
  )
  const store = redux.createStore(ethicsReducers, composeEnhancers(redux.applyMiddleware(epicMiddleware)))
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
