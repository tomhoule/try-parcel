import { actionCreatorFactory } from 'typescript-fsa'
import * as rpc from '../rpc/yacchauyo_pb'

const factory = actionCreatorFactory('texts')

export const texts = {
  fetchIndex: factory.async<{}, rpc.Text, {}>('FETCH_INDEX')
}
