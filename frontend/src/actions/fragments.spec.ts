import { FragmentsQuery } from '../rpc/yacchauyo_pb'
import { Ok, Err, mockEffects } from '../prelude'
import { Yacchauyo } from '../rpc/yacchauyo_pb_service'
import * as actions from './fragments'
import { Code } from 'grpc-web-client/dist/Code'

describe('actions/fragments', () => {
  describe('queryTask', () => {
    const action = actions.fragments.query.started(new FragmentsQuery())

    it('works', async () => {
      const effects = { ...mockEffects, rpcCall: jest.fn() }
      const response = new FragmentsQuery()
      const put = jest.fn()
      response.setPrefix('meow')
      effects.rpcCall.mockReturnValueOnce(Promise.resolve(new Ok(response)))

      const result = await actions.queryTaskInner(effects)(action.payload)(put, jest.fn())

      expect(effects.rpcCall).toHaveBeenCalledWith(Yacchauyo.QueryFragments, action.payload)
      expect(result).toEqual(new Ok(response.toObject()))
      expect(put).toHaveBeenCalledWith(actions.fragments.query.done({
        params: action.payload,
        result: response.toObject(),
      }))
    })

    it('can also fail', async () => {
      const effects = { ...mockEffects, rpcCall: jest.fn() }
      const put = jest.fn()
      const response: any = { status: Code.NotFound }
      effects.rpcCall.mockReturnValueOnce(Promise.resolve(new Err(response)))

      const result = await actions.queryTaskInner(effects)(action.payload)(put, jest.fn())

      expect(effects.rpcCall).toHaveBeenCalledWith(Yacchauyo.QueryFragments, action.payload)
      expect(result).toEqual(new Err(response))
      expect(put).toHaveBeenCalledWith(actions.fragments.query.failed({
        error: response,
        params: action.payload,
      }))
    })
  })
})
