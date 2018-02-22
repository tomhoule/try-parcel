import * as React from 'react'
import { connect } from 'react-redux'
import * as proto from '../rpc/yacchauyo_pb'
import * as actions from '../actions/fragments'

interface StateProps {
  fragments: proto.FragmentsQuery.AsObject
}

interface DispatchProps {
  query: typeof actions.queryTask

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
    fragments: state.fragments.query,
  }),
  {
    query: actions.queryTask,
  },
)(Reader)
