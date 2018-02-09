import * as React from 'react'
import Form from './Form'

interface State {
  title: string
  slug: string
  authors: string
  description: string
}

interface NewText {
  title: string
  slug: string
  authors: string
  description: string
}

export default class TextForm extends React.Component<{}, State> {
  state = {
    title: '',
    slug: '',
    authors: '',
    description: '',
  }

  submit = () => {
    fetch('/t', {
      method: 'post',
      body: JSON.stringify(this.state),
      headers: {
        'Content-Type': 'application/json',
      },
    })
  }

  render() {
    return (
      <Form
       onChange={(change: any) => this.setState(change)}
       submit={this.submit}
      >
        <label>title</label>
        <input name='title' type='text' />
        <label>slug</label>
        <input name='slug' type='text' />
        <label>authors</label>
        <input name='authors' type='text' />
        <label>description</label>
        <input name='description' type='text' />

        <br />
        <div>
          <pre>{JSON.stringify(this.state)}</pre>
        </div>
      </Form>)
  }
}
