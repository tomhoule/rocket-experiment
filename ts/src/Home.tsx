import * as React from 'react'
import { bindActionCreators } from 'redux'
import { connect } from 'react-redux'
import { Link, match } from 'react-router-dom'
import * as a from './actions'
import * as api from 'api-types'
import { AppState } from './reducers'

interface StateProps {
    schema: api.RepositoryEthicsSchema | null
}

interface DispatchProps {
    getSchema: typeof a.getSchema.started
}

interface OwnProps {
    match: match<{}>
}

type Props = StateProps & DispatchProps & OwnProps

export const Home = (props: Props) =>
    <div>
        <button onClick={() => props.getSchema({})}>Get the schema</button>
        <div>{props.schema && JSON.stringify(props.schema)}</div>
        <p>
            Choose:
        </p>
        <Link className='book' to='/ethica'>
            Ethica more geometrico demonstrata
        </Link>
        <p>
        </p>
    </div>


export default connect<StateProps, DispatchProps, OwnProps>(
    (state: AppState) => ({
        schema: state.schema,
    }),
    dispatch => bindActionCreators({
        getSchema: a.getSchema.started,
    }, dispatch)
)(Home)
