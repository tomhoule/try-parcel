import * as React from 'react'
import { connect } from 'react-redux'
import { RouteProps, RouteComponentProps } from 'react-router'
import { css } from 'emotion'
import SchemaEditor from './SchemaEditor'
import { Schema, Text } from '../rpc/yacchauyo_pb'
import { patchSchemaTask } from '../actions/schemas'
import { fetchTask, texts } from '../actions/texts'
import { Yacchauyo } from '../rpc/yacchauyo_pb_service'
import { rpcCall, Result } from '../prelude'
import TextForm from './TextForm'

const styles = css({
  ':hover': { color: 'red' },
  'color': 'green',
 })

interface StateProps {
  text: TextSingle | null
}

interface DispatchProps {
  fetchText: typeof fetchTask
  patchSchema: typeof patchSchemaTask
  receiveText: typeof texts.receive
}

interface OwnProps extends RouteComponentProps<{ textId: string }> {
}

type Props = StateProps & DispatchProps & OwnProps

export class EditText extends React.Component<Props> {
  componentWillMount() {
    this.props.fetchText(this.props.match.params.textId)
  }

  patchText = async (text: Text): Promise<Result<Text, RpcFailure>> => {
    const res = await rpcCall(Yacchauyo.PatchText, text)
    return res
      .map(text => {
        this.props.receiveText(text)
        return text
      })
  }

  render() {
    if (!this.props.text) { return 'loading...' }
    return (
      <>
        <h1 className={styles}>{this.props.text.text.title}</h1>
        <TextForm
          text={this.props.text.text}
          submit={this.patchText}
        />
        <SchemaEditor
          patchSchema={this.props.patchSchema}
          schema={this.props.text.schema}
        />
      </>
    )
  }
}

export default connect<StateProps, DispatchProps, OwnProps, AppState>(
  state => ({
    text: state.texts.single,
  }),
  {
    fetchText: fetchTask,
    patchSchema: patchSchemaTask,
    receiveText: texts.receive,
  },
)(EditText)
