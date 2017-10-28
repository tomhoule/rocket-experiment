import * as React from 'react'
import { connect } from 'react-redux'
import { AppState } from './reducers'
import { bindActionCreators } from 'redux'
import { Link } from 'react-router-dom'
import * as a from './actions'
import styles = require('../home.scss')

interface StateProps {
  editions: Edition[]
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
      <div className={styles.home}>
        <h2>Editions</h2>
        <div className={styles.books}>
          {this.props.editions.map(ed =>
            <Link key={ed.slug} to={`/ethics/${ed.slug}`}>
              {ed.title}
            </Link>)}
        </div>
      </div>)
  }
}

export default connect<StateProps, DispatchProps, OwnProps>(
  (state: AppState) => ({
    editions: state.editions.index,
  }),
  { getEditions: a.getEditions.started }
)(Editions)
