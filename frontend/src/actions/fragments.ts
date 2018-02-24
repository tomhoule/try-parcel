import { actionCreatorFactory } from 'typescript-fsa'
import * as proto from '../rpc/yacchauyo_pb'
import { buckle, rpcCall, bindThunk, effects } from '../prelude'
import { Yacchauyo } from '../rpc/yacchauyo_pb_service'

const factory = actionCreatorFactory('fragments')

export const fragments = {
  query: factory.async<proto.FragmentsQuery, proto.FragmentsQuery.AsObject, RpcFailure>('QUERY'),
}

export const queryTaskInner = (eff: typeof effects) => buckle(
  fragments.query,
  async (action, getState, dispatch) => {
    const res = await eff.rpcCall(Yacchauyo.QueryFragments, action.payload)
    return res.map(result => result.toObject())
  },
)

export const queryTask = bindThunk(queryTaskInner(effects))
