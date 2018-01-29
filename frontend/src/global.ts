import * as proto from './rpc/yacchauyo_pb'

declare global {
  type TextEntry = string

  interface TextsState {
    index: proto.Texts.AsObject
  }

  interface AppState {
    texts: TextsState
  }
}
