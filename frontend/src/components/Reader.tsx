import * as React from 'react'
import { RouteComponentProps } from 'react-router'
import { connect } from 'react-redux'
import * as proto from '../rpc/yacchauyo_pb'
import * as actions from '../actions/fragments'
import { fetchTask } from '../actions/texts'

interface StateProps {
  fragments: proto.FragmentsQuery.AsObject | null
  text: TextSingle | null
}

interface DispatchProps {
  fetchText: typeof fetchTask
  query: typeof actions.queryTask
}

interface OwnProps extends RouteComponentProps<{ textId: string }> {
  rootPath: string
}

type Props = StateProps & DispatchProps & OwnProps

export class Reader extends React.Component<Props> {
  componentWillMount() {
    if (!this.props.text || this.props.text.text.id !== this.props.match.params.textId) {
      this.props.fetchText(this.props.match.params.textId)
    }

    const path = this.path()
    if (path) {
      const query = new proto.FragmentsQuery()
      query.setPrefix(path)
      this.props.query(query)
    }
  }

  path = (): string | null => {
    const queryParams = new URLSearchParams()
    return queryParams.get('path')
  }

  render() {
    const { fragments, text } = this.props
    if (!fragments || !text) { return 'loading...' }
    return (<div />)
  }
}

export default connect<StateProps, DispatchProps, OwnProps, AppState>(
  state => ({
    fragments: state.fragments.query,
    text: state.texts.single,
  }),
  {
    fetchText: fetchTask,
    query: actions.queryTask,
  },
)(Reader)
