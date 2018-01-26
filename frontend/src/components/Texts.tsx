import * as React from 'react'
import * as ReactRouter from 'react-router'
import { connect } from 'react-redux'

import { texts } from '../actions/texts'

interface DispatchProps {
  fetchIndex: typeof texts.fetchIndex.started
}

type OwnProps = ReactRouter.RouteProps
type Props = DispatchProps & OwnProps

export class Texts extends React.Component<Props> {
  componentWillMount() {
    this.props.fetchIndex({})
  }

  render() {
    const loc = this.props.location
    return (
      <div>
        This is the texts index: {loc ? loc.pathname : 'meh'}
      </div>
    )
  }
}

export default connect<{}, DispatchProps, OwnProps>(
  null,
  {
    fetchIndex: texts.fetchIndex.started,
  }
)(Texts)
