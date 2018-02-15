import { actionCreatorFactory } from 'typescript-fsa'
import * as proto from '../rpc/yacchauyo_pb'
import { buckle, rpcCall } from '../prelude'
import * as backend from '../rpc/yacchauyo_pb_service'

const factory = actionCreatorFactory('texts')

export const texts = {
  create: factory.async<proto.Text, proto.Text.AsObject, RpcFailure>('CREATE'),
  fetchIndex: factory.async<proto.TextsQuery, proto.Texts.AsObject, RpcFailure>('FETCH_INDEX'),
  patch: factory.async<proto.Text, proto.Text.AsObject, RpcFailure>('PATCH'),
}

export const fetchIndexTask = buckle(
  texts.fetchIndex,
  async (action) => {
    const result = await rpcCall(backend.Yacchauyo.TextsIndex, action.payload)
    return result.map(res => res.toObject())
  })
