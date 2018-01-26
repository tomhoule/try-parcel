import * as rpc from './rpc/yacchauyo_pb'

type TextEntry = string

interface TextsState {
  index: rpc.Text[]
}

interface AppState {
  texts: TextsState
}
