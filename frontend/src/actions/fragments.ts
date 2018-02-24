import { actionCreatorFactory } from 'typescript-fsa'
import * as proto from '../rpc/yacchauyo_pb'
import { buckle, rpcCall, bindThunk } from '../prelude'
import { Yacchauyo } from '../rpc/yacchauyo_pb_service'

const factory = actionCreatorFactory('fragments')

export const fragments = {
  query: factory.async<proto.FragmentsQuery, proto.FragmentsQuery.AsObject, RpcFailure>('QUERY'),
}

export const queryTask = (call: typeof rpcCall) => buckle(
  fragments.query,
  async (action, getState, dispatch) => {
    const res = await call(Yacchauyo.QueryFragments, action.payload)
    return res.map(result => result.toObject())
  },
)
