import { actionCreatorFactory } from 'typescript-fsa'
import * as proto from '../rpc/yacchauyo_pb'
import { buckle, rpcCall, bindThunk } from '../prelude'
import { Yacchauyo } from '../rpc/yacchauyo_pb_service'

const factory = actionCreatorFactory('schemas')

export const schemas = {
  patchSchema: factory.async<proto.Schema, proto.Schema.AsObject, RpcFailure>('PATCH'),
  textSchema: factory.async<proto.TextsQuery, proto.Schema.AsObject, RpcFailure>('FETCH'),
}

export const textSchemaInner = (call: typeof rpcCall) => buckle(
  schemas.textSchema,
  async (action) => {
    const response = await call(Yacchauyo.TextSchema, action.payload)
    return response.map(res => res.toObject())
  },
)

export const textSchemaTask = bindThunk(textSchemaInner(rpcCall))

export const patchSchemaTask = buckle(
  schemas.patchSchema,
  async (action) => {
    const response = await rpcCall(Yacchauyo.PatchSchema, action.payload)
    return response.map(res => res.toObject())
  },
)
