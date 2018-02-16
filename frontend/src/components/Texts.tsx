import * as React from 'react'
import { connect } from 'react-redux'
import { Route, RouteProps, Switch } from 'react-router'
import { Link } from 'react-router-dom'
import { texts, fetchIndexTask } from '../actions/texts'
import * as proto from '../rpc/yacchauyo_pb'
import CreateText from './CreateText'

interface DispatchProps {
  fetchIndex: typeof fetchIndexTask
}

interface StateProps {
  texts: proto.Text.AsObject[]
}

type OwnProps = RouteProps
type Props = StateProps & DispatchProps & OwnProps

export class Texts extends React.Component<Props> {
  componentWillMount() {
    this.props.fetchIndex(new proto.TextsQuery())
  }

  render() {
    const loc = this.props.location
    return (
      <div>
        <Link to='/texts/new'>New texts</Link>
        {this.props.texts.map(text =>
          <div key={text.id}>
            {text.title}
          </div>)}
        <Switch>
          <Route path='/texts/new' component={CreateText} />
        </Switch>
      </div>
    )
  }
}

export default connect<StateProps, DispatchProps, OwnProps, AppState>(
  state => ({
    texts: state.texts.index.textsList || [],
  }),
  {
    fetchIndex: fetchIndexTask,
  },
)(Texts)
