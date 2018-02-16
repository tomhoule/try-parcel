import * as React from 'react'
import { connect } from 'react-redux'
import Form from './Form'
import { createTask, patchTask } from '../actions/texts'
import * as proto from '../rpc/yacchauyo_pb'

interface DispatchProps {
  createText: typeof createTask
  patch: typeof patchTask
}

interface OwnProps {
  params: { textId?: string }
}

type Props = DispatchProps & OwnProps
type State = Partial<proto.Text.AsObject>

export class CreateText extends React.Component<Props, State> {
  state: State = {}

  submit = () => {
    const payload = new proto.Text()
    payload.setAuthors(this.state.authors || '')
    payload.setDescription(this.state.description || '')
    payload.setSlug(this.state.slug || '')
    payload.setTitle(this.state.title || '')

    if (this.props.params.textId) {
      payload.setId(this.props.params.textId)
      this.props.patch(payload)
    } else {
      this.props.createText(payload)
    }
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

export default connect(
  undefined,
  {
    createText: createTask,
  },
)(CreateText)
