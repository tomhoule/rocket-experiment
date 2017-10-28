import * as React from 'react'
import { Link, match } from 'react-router-dom'
import * as api from 'api-types'
import * as a from './actions'
import { bindActionCreators } from 'redux'
import { connect } from 'react-redux'
import styles = require('./ethics.scss')
import Fragment from './Fragment'

interface StateProps {
  schema: api.RepositoryEthicsSchema | null
}

interface DispatchProps {
  getSchema: typeof a.getSchema.started
  getFragments: typeof a.getFragments.started
}

interface OwnProps {
  match: match<{ editionSlug: string }>
}

type Props = StateProps & DispatchProps & OwnProps

export class Ethics extends React.Component<Props, {}> {
  componentWillMount() {
    this.props.getSchema({})
    this.props.getFragments({ slug: this.props.match.params.editionSlug })
  }

  renderNode = (node: api.EthicsSchemaNode, idx: number): React.ReactElement<any> => {
    const num = node.num ? node.num : ''
    const { editionSlug } = this.props.match.params
    return (
      <div
        className={styles.node}
        key={`${node.node_type}${node.num}${idx}`}
      >
        <div>
          {node.node_type && `${node.node_type} ${num}`}
          {node.title && `--- Title: ${node.title}`}
        </div>
        <Fragment editionSlug={editionSlug} path='meh' />
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
    getFragments: a.getFragments.started,
  }, dispatch)
)(Ethics)
