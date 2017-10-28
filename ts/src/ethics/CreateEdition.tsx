import * as React from 'react'
import * as a from './actions'
import Form from 'Form'
import { connect } from 'react-redux'
import { bindActionCreators } from 'redux'
import { AppState } from './reducers'
import shellStyles = require('../shell.scss')

interface DispatchProps {
  setChanges: typeof a.setChanges
  mergeChanges: typeof a.mergeChanges
  create: typeof a.create.started
}

interface StateProps {
  changes: Partial<Edition>
  errors: Errors
}

const langOptions = [
  { value: '', label: '---' },
  { value: 'de', label: 'de' },
  { value: 'en', label: 'en' },
  { value: 'es', label: 'es' },
  { value: 'fr', label: 'fr' },
  { value: 'la', label: 'la' },
  { value: 'ru', label: 'ru' },
  { value: 'zh', label: 'zh' },
]

interface OwnProps {}

type Props = StateProps & DispatchProps & OwnProps

export class CreateEdition extends React.Component<Props, {}> {
  componentWillMount() {
    this.props.setChanges({})
  }

  submit = () => {
    const { create, changes } = this.props
    create(changes)
  }

  render() {
    const { changes, errors } = this.props
    return (
      <div className={shellStyles.container}>
        <Form
          changes={changes}
          errors={errors}
          elements={[
            { type: 'heading', content: 'Create an edition' },
            { type: 'text', label: 'Title', name: 'title' },
            { type: 'text', label: 'Slug', name: 'slug' },
            { type: 'text', label: 'Editor', name: 'editor'},
            { type: 'number', label: 'Year', name: 'year' },
            { type: 'select', label: 'Lang', name: 'language_code', options: langOptions },
          ]}
          mergeChanges={this.props.mergeChanges}
          submit={this.submit}
        />
      </div>
    )
  }
}

export default connect<StateProps, DispatchProps, OwnProps>(
  (state: AppState) => ({
    changes: state.editions.changes,
    errors: state.editions.errors,
  }),
  d => bindActionCreators({
    mergeChanges: a.mergeChanges,
    setChanges: a.setChanges,
    create: a.create.started,
  }, d)
)(CreateEdition)
