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
import * as ethicsReducers from './ethics/reducers'
import * as api from 'api-types'
import { InjectedDependencies } from './types'
import createHistory from 'history/createBrowserHistory'
import { ConnectedRouter, routerReducer, routerMiddleware } from 'react-router-redux'

const composeEnhancers =
    (window as any).__REDUX_DEVTOOLS_EXTENSION_COMPOSE__
    ? (window as any).__REDUX_DEVTOOLS_EXTENSION_COMPOSE__
    : redux.compose

const rootEpic = ethicsRootEpic

const history = createHistory()

export const reducers = redux.combineReducers<ethicsReducers.AppState>({
    schema: ethicsReducers.schemaReducer,
    editions: ethicsReducers.editionsReducer,
    router: routerReducer
})

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
  const store = redux.createStore(reducers, composeEnhancers(redux.applyMiddleware(epicMiddleware, routerMiddleware(history as any))))
  const root = document.getElementById('react-root')
  render(
    <BrowserRouter>
      <Provider store={store}>
        <ConnectedRouter history={history as any}>
          <Route path='/' component={App} />
        </ConnectedRouter>
      </Provider>
    </BrowserRouter>,
    root)
}

main()
