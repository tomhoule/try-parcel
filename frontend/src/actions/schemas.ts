import { actionCreatorFactory } from 'typescript-fsa'
import * as proto from '../rpc/yacchauyo_pb'
import { buckle, rpcCall } from '../prelude'
import { Yacchauyo } from '../rpc/yacchauyo_pb_service';

const factory = actionCreatorFactory('schemas')

export const schemas = {
  patchSchema: factory.async<proto.Schema, proto.Schema, {}>('PATCH'),
  textSchema: factory.async<proto.TextsQuery, proto.Schema.AsObject, {}>('FETCH'),
}

export const textSchemaTask = buckle(
  schemas.textSchema,
  async (action) => {
    const response = await rpcCall(Yacchauyo.TextSchema, action.payload)
    return response.map(res => res.toObject())
  }
)
