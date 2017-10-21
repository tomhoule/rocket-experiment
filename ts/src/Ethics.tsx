import * as React from 'react'
import { Link, match } from 'react-router-dom'
import * as api from 'api-types'
import * as a from './actions'
import { AppState } from './reducers'
import { bindActionCreators } from 'redux'
import { connect } from 'react-redux'
import styles = require('./ethics.scss')

interface StateProps {
  schema: api.RepositoryEthicsSchema | null
}

interface DispatchProps {
  getSchema: typeof a.getSchema.started
}

interface OwnProps {
  match: match<{ editionSlug: string }>
}

type Props = StateProps & DispatchProps & OwnProps

export class Ethics extends React.Component<Props, {}> {
  componentWillMount() {
    this.props.getSchema({})
  }

  renderNode = (node: api.EthicsSchemaNode, idx: number): React.ReactElement<any> => {
    return (
      <div
        className={styles.node}
        key={`${node.node_type}${node.num}${idx}`}
      >
        <div>
          {node.node_type} {node.num === 0 ? '' : node.num}
        </div>
        <div>
          {(node.children || []).map(this.renderNode)}
        </div>
      </div>)
  }

  render() {
    const { schema } = this.props
    return (
      <div>
        <div>{schema && (schema.parts || []).map(this.renderNode)}</div>
      </div>
    )
  }
}

export default connect<StateProps, DispatchProps, OwnProps>(
    (state: AppState) => ({
        schema: state.schema,
    }),
    dispatch => bindActionCreators({
        getSchema: a.getSchema.started,
    }, dispatch)
)(Ethics)
