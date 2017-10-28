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

function buildPath(
  prefix: string,
  suffix: SchemaNode,
): string {
  const title = typeof suffix.node_type === 'string'
    ? suffix.node_type
    : suffix.node_type.Scope
  const num = suffix.num ? `(${suffix.num})` : ''
  return `${prefix}${prefix.length ? ':' : ''}${title.toLowerCase()}${num}`
}

interface DecoratedNode extends SchemaNode {
  path: string
}

export class Ethics extends React.Component<Props, {}> {
  componentWillMount() {
    this.props.getSchema({})
    this.props.getFragments({ slug: this.props.match.params.editionSlug })
  }

  renderNode = (node: DecoratedNode, idx: number): React.ReactElement<any> => {
    const num = node.num ? node.num : ''
    const { editionSlug } = this.props.match.params
    return (
      <div
        className={styles.node}
        key={`${node.path}${idx}`}
      >
        <div>
          {node.node_type && `${renderNodeType(node.node_type)} ${num}`}
        </div>
        <div>
          path: {node.path}
        </div>
        <Fragment editionSlug={editionSlug} path={node.path} />
        <div>
          {node.children.map((child, idx) => this.renderNode({
            ...child,
            path: buildPath(node.path, child),
          }, idx))}
        </div>
      </div>)
  }

  render() {
    const { schema } = this.props
    return (
      <div>
        <div>
          {schema && schema.children.map((node, idx) => this.renderNode({
            ...node,
            path: buildPath('', node),
          }, idx))}
        </div>
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
