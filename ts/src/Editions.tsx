import * as React from 'react'
import { connect } from 'react-redux'
import { AppState } from 'reducers'
import { bindActionCreators } from 'redux'
import { Link } from 'react-router-dom'
import * as api from 'typescript-fetch-api/api'

import * as a from 'actions'

interface StateProps {
    editions: api.RepositoryEdition[]
}

interface DispatchProps {
    getEditions: typeof a.getEditions.started
}

interface OwnProps {}

type Props = StateProps & DispatchProps

export class Editions extends React.Component<Props, never> {
    componentWillMount() {
        this.props.getEditions({})
    }

    render() {
        return (
            <div>
                <h2>Editions</h2>
                {this.props.editions.map(ed =>
                    <Link key={ed.slug} to={`/ethica/editions/${ed.slug}`}>
                        {ed.title}
                    </Link>)}
            </div>)
    }
}

export default connect<StateProps, DispatchProps, OwnProps>(
    (state: AppState) => ({
        editions: state.editions.index,
    }),
    dispatch => bindActionCreators({
        getEditions: a.getEditions.started,
    }, dispatch)
)(Editions)
