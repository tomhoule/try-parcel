import * as React from 'react'
import { connect } from 'react-redux'
import { RouteComponentProps } from 'react-router'
import Form from './Form'
import * as proto from '../rpc/yacchauyo_pb'
import { Result, rpcCall } from '../prelude'
import { Yacchauyo } from '../rpc/yacchauyo_pb_service'
import { TextForm } from './TextForm'
import * as texts from '../actions/texts'

interface DispatchProps {
  create: typeof texts.createTask
}

interface OwnProps extends RouteComponentProps<{}> {}

type Props = DispatchProps & OwnProps

interface State {}

export class CreateText extends React.Component<Props, State> {
  submit = (text: proto.Text): Promise<Result<proto.Text.AsObject, RpcFailure>> => {
    return this.props.create(text)
  }

  render() {
    return (
      <TextForm submit={this.submit} />
    )
  }
}

export default connect<{}, DispatchProps, OwnProps, AppState>(
  undefined,
  {
    create: texts.createTask,
  },
)(CreateText)
