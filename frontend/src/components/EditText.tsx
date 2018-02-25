import * as React from 'react'
import { connect } from 'react-redux'
import { RouteProps, RouteComponentProps } from 'react-router'
import { css } from 'emotion'
import SchemaEditor from './SchemaEditor'
import { Schema, Text } from '../rpc/yacchauyo_pb'
import { patchSchemaTask } from '../actions/schemas'
import { fetchTask, texts, patchTask } from '../actions/texts'
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
  patchText: typeof patchTask
}

interface OwnProps extends RouteComponentProps<{ textId: string }> {
}

type Props = StateProps & DispatchProps & OwnProps

export class EditText extends React.Component<Props> {
  componentWillMount() {
    this.props.fetchText(this.props.match.params.textId)
  }

  patchText = async (text: Text): Promise<Result<Text.AsObject, RpcFailure>> => {
    return this.props.patchText(text)
  }

  render() {
    const { text, match } = this.props
    if (!text || text.text.id !== match.params.textId) { return 'loading...' }
    return (
      <>
        <h1 className={styles}>{text.text.title}</h1>
        <TextForm
          text={text.text}
          submit={this.patchText}
        />
        <SchemaEditor
          patchSchema={this.props.patchSchema}
          schema={text.schema}
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
    patchText: patchTask,
  },
)(EditText)
