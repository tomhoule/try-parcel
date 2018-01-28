import { actionCreatorFactory } from 'typescript-fsa'
import * as proto from '../rpc/yacchauyo_pb'

const factory = actionCreatorFactory('schemas')

export const schemas = {
  patchSchema: factory.async<proto.Schema, proto.Schema, {}>('PATCH'),
  textSchema: factory.async<proto.TextsQuery, proto.Schema, {}>('FETCH'),
}
