import * as actions from './texts'
import { Text } from '../rpc/yacchauyo_pb'
import { Ok, mockEffects, Err } from '../prelude'
import { Code } from 'grpc-web-client/dist/Code'

describe('actions/texts', () => {
  describe('createTask', () => {
    it('works as expected', async () => {
      const client = jest.fn()
      const effects = { ...mockEffects, rpcCall: client }
      const put = jest.fn()
      const response = new Text()
      response.setId('created-id')
      client.mockReturnValueOnce(Promise.resolve(new Ok(response)))
      const payload = new Text()
      payload.setId('abcmytext')
      const result = await actions.createTaskInner(effects)(payload)(put, jest.fn())

      expect(result.isOk()).toBe(true)
      result.map(text => expect(text.id).toBe('created-id'))
      expect(put).toHaveBeenCalledWith(actions.texts.create.done({
        params: payload,
        result: response.toObject(),
      }))
    })

    it('fails as expected', async () => {
      const effects = { ...mockEffects, rpcCall: jest.fn() }
      const put = jest.fn()
      const error = { status: Code.AlreadyExists, statusMessage: 'Meh' }
      effects.rpcCall.mockReturnValueOnce(Promise.resolve(new Err(error)))
      const payload = new Text()
      const res = await actions.createTaskInner(effects)(payload)(put, jest.fn())

      expect(res.isErr()).toBe(true)
      res.mapErr(err => expect(err).toEqual(error))
      expect(put).toHaveBeenCalledWith(actions.texts.create.started(payload))
      expect(put).toHaveBeenCalledWith(actions.texts.create.failed({
        error,
        params: payload,
      }))
    })
  })
})
