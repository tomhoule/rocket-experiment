import { ActionsObservable, combineEpics, Epic } from 'redux-observable'
import * as actions from './actions'
import * as Rx from 'rxjs/Rx'
import { AsyncActionCreators } from 'typescript-fsa'
import 'typescript-fsa-redux-observable'
import * as msgs from 'rpc/repository_pb'
import { EthicsRepository } from 'rpc/repository_pb_service'
import { InjectedDependencies } from './types'
import { AppState } from './reducers'
import { rejoice } from 'epic-utils'
import * as pb from 'pbutils'

type AppEpic = Epic<Action, AppState, InjectedDependencies>

const schemaEpic: AppEpic= (action$, store, d) =>
    action$
        .ofAction(actions.getSchema.started)
        .mergeMap(async ({ payload: params }) =>
            await d.rpc<
                typeof EthicsRepository.GetSchema.requestType.prototype,
                typeof EthicsRepository.GetSchema.responseType.prototype
            >(EthicsRepository.GetSchema, new msgs.GetSchemaParams()).
                then(rejoice(actions.getSchema.done, params)))

const editions: AppEpic = (action$, store, d) =>
    action$
        .ofAction(actions.getEditions.started)
        .mergeMap(async ({ payload: params }) => {
            const result = await d.rpc<
                    typeof EthicsRepository.GetEditions.requestType.prototype,
                    typeof EthicsRepository.GetEditions.responseType.prototype
                >(EthicsRepository.GetEditions, new msgs.GetEditionsParams())
            return actions.getEditions.done({ params, result: result.toObject() })
        })

export const rootEpic = combineEpics(schemaEpic, patchEditionEpic, createEdition, editions)
