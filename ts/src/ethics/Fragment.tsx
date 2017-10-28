import * as React from 'react'
import { Link, match } from 'react-router-dom'
import * as a from './actions'
import { AppState } from './reducers'
import { bindActionCreators } from 'redux'
import { connect } from 'react-redux'
import styles = require('./ethics.scss')

interface StateProps {}

interface DispatchProps {}

interface OwnProps {
  editionSlug: string
  path: string
}

type Props = StateProps & DispatchProps & OwnProps

interface State {
  editing: boolean
}

export class Fragment extends React.Component<Props, State> {
  constructor(props: Props) {
    super(props)
    this.state = { editing: false }
  }

  toggleEdit = (): void => this.setState({ editing: !this.state.editing })

  render() {
    const { editing } = this.state
    if (!editing) {
      return (
        <p>
          This is a fragment <button onClick={this.toggleEdit}>Edit</button>
        </p>)
    } else {
      return (
        <textarea onBlur={this.toggleEdit}>This is an editable fragment</textarea>)
    }
  }
}

export default connect<StateProps, DispatchProps, OwnProps>(
  () => ({
  })
)(Fragment)
