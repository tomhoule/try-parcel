import * as actions from './texts'
import { Text } from '../rpc/yacchauyo_pb'
import { Ok } from '../prelude'
import { push } from 'react-router-redux'

describe('actions/texts', () => {
  describe('createTask', () => {
    it('works as expected', async () => {
      const client = jest.fn()
      const put = jest.fn()
      const response = new Text()
      response.setId('created-id')
      client.mockReturnValueOnce(Promise.resolve(new Ok(response)))
      const payload = new Text()
      payload.setId('abcmytext')
      const result = await actions.createTaskInner(client)(payload)(put, jest.fn())

      expect(result.isOk()).toBe(true)
      result.map(text => expect(text.id).toBe('created-id'))
      expect(put).toHaveBeenCalledWith(actions.texts.create.done({
        params: payload,
        result: response.toObject(),
      }))
    })
  })
})
