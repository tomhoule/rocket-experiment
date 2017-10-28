import * as React from 'react'
import { Link, match } from 'react-router-dom'
import * as a from './actions'
import { bindActionCreators } from 'redux'
import { connect } from 'react-redux'
import styles = require('./ethics.scss')
import Fragment from './Fragment'

interface StateProps {
  schema: EthicsSchema | null
}

interface DispatchProps {
  getSchema: typeof a.getSchema.started
  getFragments: typeof a.getFragments.started
}

interface OwnProps {
  match: match<{ editionSlug: string }>
}

type Props = StateProps & DispatchProps & OwnProps

function renderNodeType(nt: NodeType): string {
  if (typeof nt === 'string') {
    return nt
  } else {
    return nt.Scope
  }
}

export class Ethics extends React.Component<Props, {}> {
  componentWillMount() {
    this.props.getSchema({})
    this.props.getFragments({ slug: this.props.match.params.editionSlug })
  }

  renderNode = (node: SchemaNode, idx: number): React.ReactElement<any> => {
    const num = node.num ? node.num : ''
    const { editionSlug } = this.props.match.params
    return (
      <div
        className={styles.node}
        key={`${node.node_type}${node.num}${idx}`}
      >
        <div>
          {node.node_type && `${renderNodeType(node.node_type)} ${num}`}
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
        <div>{schema && schema.children.map(this.renderNode)}</div>
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
