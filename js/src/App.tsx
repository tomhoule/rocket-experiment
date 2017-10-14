import * as React from 'react'
import { connect } from 'react-redux'
import { bindActionCreators } from 'redux'
import * as a from './actions'
import { AppState } from './reducers'
import * as api from 'rpc/repository_pb'

interface StateProps {
    schema: api.EthicsSchema.AsObject | null
    status: string
}

interface DispatchProps {
    changeStatus: typeof a.changeStatus
    getSchema: typeof a.getSchema.started
}

type Props = StateProps & DispatchProps

export const App = (props: Props) => (
    <div>
        <div>Hullo --- its a me, {props.status}</div>
        <button onClick={() => props.changeStatus({ newStatus: '还行' })}>Click a me, Mario</button>
        <button onClick={() => props.getSchema({})}>Get the schema</button>
        <div>{props.schema && JSON.stringify(props.schema)}</div>
    </div>)

export default connect<StateProps, DispatchProps>(
    (state: AppState) => ({
        status: state.status,
        schema: state.schema,
    }),
    dispatch => bindActionCreators({
        changeStatus: a.changeStatus,
        getSchema: a.getSchema.started,
    }, dispatch)
)(App)
