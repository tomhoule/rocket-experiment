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
import 'rxjs'
import { reducers } from './reducers'

const composeEnhancers = (window as any).__REDUX_DEVTOOLS_EXTENSION_COMPOSE__ || redux.compose

function main() {
    const epicMiddleware = createEpicMiddleware(rootEpic, {
        dependencies: {
            get: epicUtils.get,
        },
    })
    const store = redux.createStore(reducers, composeEnhancers(redux.applyMiddleware(epicMiddleware)))
    const root = document.getElementById('react-root')
    render(<Provider store={store}><App /></Provider>, root)
}

main()

import { grpc, Code, Metadata } from 'grpc-web-client'
import { GetSchemaParams, EthicsSchema } from './rpc/repository_pb'
import { EthicsRepository } from './rpc/repository_pb_service'

// console.log("trying grpc")

// const schemaRequest = new GetSchemaParams()
// grpc.invoke(EthicsRepository.GetSchema, {
//     request: schemaRequest,
//     host: 'https://localhost:8443',
//     onMessage: (message: EthicsSchema) => {
//         console.log("got schema: ", message.toObject());
//     },
//     onEnd: (code: Code, msg: string | undefined, trailers: Metadata) => {
//         if (code == Code.OK) {
//             console.log("all ok");
//         } else {
//             console.log("hit an error", code, msg, trailers);
//         }
//     },
// })
