import * as React from 'react'
import { connect } from 'react-redux'
import { RouteComponentProps } from 'react-router'
import Form from './Form'
import * as proto from '../rpc/yacchauyo_pb'
import { Result, rpcCall } from '../prelude'
import { Yacchauyo } from '../rpc/yacchauyo_pb_service'
import { TextForm } from './TextForm'

interface OwnProps extends RouteComponentProps<{}> {}

type Props = OwnProps

interface State {}

export class CreateText extends React.Component<Props, State> {
  submit = async (text: proto.Text): Promise<Result<proto.Text, RpcFailure>> => {
    const res = await rpcCall(Yacchauyo.CreateText, text)
    return res
      .map(text => {
        this.props.history.push('/')
        return text
      })
  }

  render() {
    return (
      <TextForm submit={this.submit} />
    )
  }
}

export default CreateText
