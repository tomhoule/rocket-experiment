import { ActionsObservable, combineEpics, Epic } from 'redux-observable'
import * as actions from './actions'
import * as Rx from 'rxjs/Rx'
import { AsyncActionCreators } from 'typescript-fsa'
import 'typescript-fsa-redux-observable'
import * as msgs from 'rpc/repository_pb'
import { EthicsRepository } from 'rpc/repository_pb_service'
import { InjectedDependencies } from './types'
import { AppState } from './reducers'

const schemaEpic: Epic<Action, AppState, InjectedDependencies> = (action$, store, d) =>
    action$
        .ofAction(actions.getSchema.started)
        .mergeMap(async ({ payload: params }) =>
            await d.rpc<
                msgs.GetSchemaParams,
                msgs.EthicsSchema
            >(EthicsRepository.GetSchema, new msgs.GetSchemaParams()).
                then(result => actions.getSchema.done(({ params, result: result.toObject() }))),
        )

export const rootEpic = combineEpics(schemaEpic)
