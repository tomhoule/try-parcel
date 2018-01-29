import { actionCreatorFactory } from 'typescript-fsa'
import * as proto from '../rpc/yacchauyo_pb'

const factory = actionCreatorFactory('texts')

export const texts = {
  create: factory.async<proto.Text, proto.Text.AsObject, {}>('CREATE'),
  fetchIndex: factory.async<proto.TextsQuery, proto.Texts.AsObject, {}>('FETCH_INDEX'),
  patch: factory.async<proto.Text, proto.Text.AsObject, {}>('PATCH'),
}
