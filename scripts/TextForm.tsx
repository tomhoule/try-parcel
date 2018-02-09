import * as React from 'react'

interface State {
  title: string
  slug: string
  authors: string
  description: string
}

export default class TextForm extends React.Component<{}, State> {
  render() {
    return (<>This is the form</>)
  }
}
