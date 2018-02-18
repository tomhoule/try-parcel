import * as React from 'react'
import { connect } from 'react-redux'
import { RouteProps } from 'react-router'
import { Link } from 'react-router-dom'
import { texts, fetchIndexTask } from '../actions/texts'
import * as proto from '../rpc/yacchauyo_pb'
import styled from 'react-emotion'

const Entry = styled(Link)`
  display: block;
  text-decoration: none;
  color: black;
  border: solid 1px rgba(0, 0, 0, .3);
  padding: .5rem;

  &:hover {
    background-color: cyan;
  }
`

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
        <Link to='/texts/new'>New text</Link>
        {this.props.texts.map(text =>
          <Entry to={`/t/${text.id}/edit`} key={text.id}>
            {text.title}
          </Entry>)}
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
