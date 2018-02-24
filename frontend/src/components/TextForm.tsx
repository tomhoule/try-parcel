import * as React from 'react'
import { connect } from 'react-redux'
import { RouteComponentProps } from 'react-router'
import Form from './Form'
import * as proto from '../rpc/yacchauyo_pb'
import { Result } from '../prelude'

interface OwnProps {
  submit: (text: proto.Text) => Promise<Result<proto.Text, RpcFailure>>
  text?: proto.Text.AsObject
}

type Props = OwnProps
type State = Partial<proto.Text.AsObject> & { err: RpcFailure | null }

export class TextForm extends React.Component<Props, State> {
  constructor(props: Props) {
    super(props)

    this.state = { ...(this.props.text || {}), err: null }
  }

  submit = async () => {
    const payload = new proto.Text()
    payload.setAuthors(this.state.authors || '')
    payload.setDescription(this.state.description || '')
    payload.setSlug(this.state.slug || '')
    payload.setTitle(this.state.title || '')

    const result = await this.props.submit(payload)

    result.mapErr(err => this.setState({ err }))
  }

  render() {
    const { err } = this.state
    return (
      <Form
       onChange={(change: any) => this.setState(change)}
       submit={this.submit}
      >
        {err && <h2>{err.statusMessage}</h2>}
        <label>title</label>
        <input name='title' type='text' value={this.state.title} />
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

export default TextForm
