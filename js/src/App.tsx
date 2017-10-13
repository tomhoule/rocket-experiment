import * as React from 'react'
import { connect } from 'react-redux'
import { bindActionCreators } from 'redux'
import * as a from './actions'

interface StateProps {
    status: string
}

interface DispatchProps {
    changeStatus: (pl: { newStatus: string }) => void
}

type Props = StateProps & DispatchProps

export const App = (props: Props) => (
    <div>
        <div>Hullo --- its a me, {props.status}</div>
        <button onClick={() => props.changeStatus({ newStatus: '还行' })}>Click a me, Mario</button>
    </div>)

export default connect<StateProps, DispatchProps>(
    (state: AppState) => ({ status: state.status }),
    dispatch => bindActionCreators({
        changeStatus: a.changeStatus,
    }, dispatch)
)(App)
