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

const patchEditionEpic: AppEpic = (action$, store, d) =>
    action$
        .ofAction(actions.patchEdition.started)
        .mergeMap(async ({ payload }) => {
            const msg = new msgs.EditionPatch()
            pb.optionalStr(msg.setTitle, payload.title)
            pb.optionalStr(msg.setEditor, payload.editor)
            pb.optionalStr(msg.setLanguageCode, payload.languageCode)
            pb.optionalI32(msg.setYear, payload.year)
            return await d.rpc<
                typeof EthicsRepository.PatchEdition.requestType.prototype,
                typeof EthicsRepository.PatchEdition.responseType.prototype
            >(EthicsRepository.PatchEdition, new msgs.EditionPatch())
                .then(rejoice(actions.patchEdition.done, payload))
        })

const createEdition: AppEpic = (action$, store, d) =>
    action$
        .ofAction(actions.createEdition.started)
        .mergeMap(async ({ payload: params }) => {
            const req = new msgs.Edition();
            req.setTitle(params.title)
            req.setSlug(params.slug)
            req.setEditor(params.editor)
            req.setLanguageCode(params.languageCode)
            req.setYear(params.year)
            const result = await d.rpc<
                    typeof EthicsRepository.CreateEdition.requestType.prototype,
                    typeof EthicsRepository.CreateEdition.responseType.prototype
                >(EthicsRepository.CreateEdition, req)
            return actions.createEdition.done({ params, result: result.toObject() })
        })

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
