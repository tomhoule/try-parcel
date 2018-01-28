import { actionCreatorFactory } from 'typescript-fsa'
import * as proto from '../rpc/yacchauyo_pb'

const factory = actionCreatorFactory('schemas')

export const schemas = {
  textSchema: factory.async<proto.TextsQuery, proto.Schema, {}>('CREATE'),
}
