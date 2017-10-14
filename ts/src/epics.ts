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

const schemaEpic: Epic<Action, AppState, InjectedDependencies> = (action$, store, d) =>
    action$
        .ofAction(actions.getSchema.started)
        .mergeMap(async ({ payload: params }) =>
            await d.rpc<
                typeof EthicsRepository.GetSchema.requestType.prototype,
                typeof EthicsRepository.GetSchema.responseType.prototype
            >(EthicsRepository.GetSchema, new msgs.GetSchemaParams()).
                then(rejoice(actions.getSchema.done, params)))

const patchEditionEpic: Epic<Action, AppState, InjectedDependencies> = (action$, store, d) =>
    action$
        .ofAction(actions.patchEdition.started)
        .mergeMap(async ({ payload }) =>
            await d.rpc<
                typeof EthicsRepository.PatchEdition.requestType.prototype,
                typeof EthicsRepository.PatchEdition.responseType.prototype
            >(EthicsRepository.PatchEdition, new msgs.EditionPatch())
                .then(rejoice(actions.patchEdition.done, payload)))

export const rootEpic = combineEpics(schemaEpic)
