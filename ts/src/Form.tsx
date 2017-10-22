import * as React from 'react'
import styles = require('./form.scss')

interface TextBase {
  extraClass?: string
  label: React.ReactElement<any> | string
  name: string
  value?: string
}

interface TextInput extends TextBase {
  type: 'text'
}

interface TextArea extends TextBase {
  type: 'textarea'
}

interface NumberInput extends TextBase {
  type: 'number'
}

interface Select {
  label: React.ReactElement<any> | string
  name: string
  options: { value: string | number | undefined, label?: string }[]
  type: 'select'
  value?: string | number | null
}

interface FormHeading {
  type: 'heading'
  content: React.ReactElement<any> | string
}

type FormElement = TextInput | TextArea | Select | FormHeading | NumberInput

function renderElement(
  elem: FormElement,
  mergeChanges: Function,
  changes: { [name: string]: any } = {},
  errors: Errors
): React.ReactElement<any> {
    switch (elem.type) {
        case 'number':
        case 'text':
          return (
            <div className={styles.field}>
              <label>{elem.label}</label>
              <input
                type={elem.type}
                onChange={event => mergeChanges({
                  [elem.name]: elem.type === 'number' ?
                    parseInt(event.target.value, 10) :
                    event.target.value
                })}
                name={elem.name}
                value={elem.value || changes[elem.name]}
              />
              {errors[elem.name] && <div style={{ color: 'red' }}>{errors[elem.name]}</div>}
            </div>)
        case 'textarea':
          return <textarea value={elem.value} />
        case 'select':
          const value = elem.value || changes[elem.name] || ''
          return (
            <div className={styles.field}>
              <label>{elem.label}</label>
                <select
                  value={value}
                  onChange={event => mergeChanges({ [elem.name]: event.target.value })}
                >
                  {elem.options.map(opt =>
                    <option value={opt.value} key={opt.value}>{opt.label}</option>)}
                </select>
            </div>)
        case 'heading':
            return (<h2>{elem.content}</h2>)
        default:
            return <span>'UNREACHABLE'</span>

    }
}

interface Props<T> {
  changes?: Partial<T>
  errors: { [key: string]: string },
  mergeChanges: (changes: Partial<T>) => any
  submit?: () => void
  elements: FormElement[]
}

export class Form<T> extends React.Component<Props<T>, never> {
  render() {
    const { changes, elements, errors, mergeChanges, submit } = this.props
    return (
      <div className={styles['form-container']}>
        {errors.other && errors.other}
        {elements.map(elem => renderElement(elem, mergeChanges, changes, errors))}
        {submit &&
            <div className={styles.submit}>
              <button onClick={submit}>Save</button>
            </div>}
      </div>
    )
  }
}

export default Form
