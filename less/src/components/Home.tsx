import * as React from 'react'
import { RouteComponentProps } from 'react-router'
import styled from 'react-emotion'
import { rpcCall } from '../prelude';
import { Yacchauyo } from '../rpc/yacchauyo_pb_service'
import { TextsQuery, Texts } from '../rpc/yacchauyo_pb'

interface Props extends RouteComponentProps<{}> { }

interface State {
  texts: Texts | null
}

const TextEntry = styled('div')({
  padding: '2em',
  color: 'tomato',
})

export class Home extends React.Component<Props, State> {
  constructor(props: Props) {
    super(props)
    this.fetchAll()
    this.state = {
      texts: null
    }
  }

  fetchAll = async () => {
    const res = await rpcCall(Yacchauyo.TextsIndex, new TextsQuery())
    res
      .map(texts => this.setState({ texts }))
      .mapErr(err => console.log('error', err))
  }

  render() {
    const { texts } = this.state
    return (
      <>
        {
          texts
            ? texts.getTextsList().map(text => <TextEntry>{text.getTitle()} </TextEntry>)
            : 'we don’t have texts'
        }
      </>
    )
  }
}
