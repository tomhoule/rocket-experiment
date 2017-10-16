import * as React from 'react'
import { connect } from 'react-redux'
import { Link } from 'react-router-dom'

interface StateProps {
    schema: api.EthicsSchema.AsObject | null
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
)
