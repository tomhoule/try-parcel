import React from 'react'
import { connect } from 'react-redux'

interface StateProps {
  fragments: Fragment[]
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
    fragments: state.
  }),
  {

  }
)(Reader)
