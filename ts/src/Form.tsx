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

interface Select {
    type: 'select'
    options: { value: string | number | null, label?: string }[]
    value: string | number | null
}

interface FormHeading {
    type: 'heading'
    content: React.ReactElement<any> | string
}

type FormElement = TextInput | TextArea | Select | FormHeading

function renderElement(
  elem: FormElement,
  mergeChanges: Function,
  changes: { [name: string]: any } = {},
): React.ReactElement<any> {
    switch (elem.type) {
        case 'text':
            return (
              <div className={styles.field}>
                <label>{elem.label}</label>
                <input
                  type='text'
                  onChange={event => mergeChanges({ [elem.name]: event.target.value })}
                  name={elem.name}
                  value={elem.value || changes[elem.name]}
                />
              </div>)
        case 'textarea':
            return <textarea value={elem.value} />
        // case 'select':
        //     return (
        //     <select value={elem.value}>
        //         {elem.options.map(opt =>
        //             <option value={opt.value} key={opt.value}>{opt.label}</option>)}
        //     </select>)
        case 'heading':
            return (<h2>{elem.content}</h2>)
        default:
            return <span>'UNREACHABLE'</span>

    }
}

interface Props<T> {
  changes?: Partial<T>
  mergeChanges: (changes: Partial<T>) => any
  submit?: () => void
  elements: FormElement[]
}

export class Form<T> extends React.Component<Props<T>, never> {
  render() {
    const { changes, elements, mergeChanges, submit } = this.props
    return (
      <div className={styles['form-container']}>
        {elements.map(elem => renderElement(elem, mergeChanges, changes))}
        {submit &&
            <div className={styles.submit}>
              <button onClick={submit}>Save</button>
            </div>}
      </div>
    )
  }
}

export default Form
