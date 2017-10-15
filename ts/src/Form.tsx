import * as React from 'react'

interface TextInput {
    extraClass?: string
    type: 'text'
    value: string
}

interface TextArea {
    extraClass?: string
    type: 'textarea'
    value: string
}

interface Select {
    type: 'select'
    options: { value: string | number | null, label?: string }[]
    value: string | number | null
}

interface FormHeading {
    type: 'heading'
    content: React.ReactElement<any>
}

type FormElement = TextInput | TextArea | Select | FormHeading

function renderElement(elem: FormElement): React.ReactElement<any> {
    switch (elem.type) {
        case 'text':
            return <input type='text' value={elem.value} />
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
    mergeChanges?: (changes: Partial<T>) => void
    submit: () => void
    elements: FormElement[]
}

export class Form<T> extends React.Component<Props<T>, never> {
    render() {
        return (
            <div>This is a form</div>
        )
    }
}
