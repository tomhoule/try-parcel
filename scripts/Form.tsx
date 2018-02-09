import * as React from 'react'

interface Props {
  onChange: (change: Record<string, any>) => void
  submit: () => void
}

export default class Form extends React.Component<Props> {

  onChange = (event: any): void => {
    this.props.onChange({ [event.target.name]: event.target.value })
  }

  onSubmit = (event: React.FormEvent<HTMLFormElement>): void => {
    event.preventDefault()
    this.props.submit()
  }

  render() {
    return (
      <form onChange={this.onChange} onSubmit={this.onSubmit}>
        {this.props.children}
        <button type='submit'>Save</button>
      </form>
    )
  }
}
