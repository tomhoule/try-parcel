import * as React from 'react'

type OnChange = any

interface TextInputProps {
  onChange: OnChange
  errors: any[]
  label?: string
  name: string
  value: string | null | undefined
}

export class TextInput extends React.Component<TextInputProps> {
  render() {
    const { name, value, onChange } = this.props
    return (
      <>
        <label htmlFor={name}>{this.props.label}</label>
        <input
          name={name}
          onChange={onChange}
          value={value || ''}
        />
      </>
    )
  }
}
