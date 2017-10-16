import * as React from 'react'

interface Props {
    label: string
    name: string
    placeholder?: string
    textarea?: boolean
    update: (val: string) => void
    value: string
}

export class TextEdit extends React.Component<Props, never> {
    render() {
        return (
            <div className='field is-horizontal'>
                <div className='field-label'>
                    <label class='label'>{this.props.label}</label>
                </div>
                <div className='field-body'>
                    <div className='field'>
                        <div className='control'>
                            <input
                                className='input'
                                name={this.props.name}
                                placeholder={this.props.placeholder}
                                type='text'
                                value={this.props.value}
                            />
                        </div>
                    </div>
                </div>
            </div>)
    }
}
