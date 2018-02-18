import * as React from 'react'
import { connect } from 'react-redux'
import { RouteProps, RouteComponentProps } from 'react-router'
import SchemaEditor from './SchemaEditor'
import { Schema, Text } from '../rpc/yacchauyo_pb'
import { patchSchemaTask } from '../actions/schemas'
import { fetchTask } from '../actions/texts'
import { css } from 'emotion'

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
}

interface OwnProps extends RouteComponentProps<{ textId: string }> {
}

type Props = StateProps & DispatchProps & OwnProps

export class EditText extends React.Component<Props> {
  componentWillMount() {
    this.props.fetchText(this.props.match.params.textId)
  }

  render() {
    if (!this.props.text) { return 'loading...' }
    return (
      <>
        <h1 className={styles}>{this.props.text.text.title}</h1>
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
  },
)(EditText)