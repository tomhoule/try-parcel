import * as React from 'react'
import { connect } from 'react-redux'
import * as proto from '../rpc/yacchauyo_pb'

interface StateProps {
  fragments: proto.Fragment.AsObject[]
}

interface DispatchProps {

}

interface OwnProps {
  rootPath: string
}

type Props = StateProps & DispatchProps

export class Reader extends React.Component<Props> {
  render() {
    return (<div />)
  }
}

export default connect<StateProps, DispatchProps, OwnProps, AppState>(
  state => ({
    fragments: state.texts.single
  }),
  {
  }
)(Reader)
