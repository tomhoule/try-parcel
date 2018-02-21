import { FragmentsQuery } from '../rpc/yacchauyo_pb'
import { Ok, Err } from '../prelude'
import { Yacchauyo } from '../rpc/yacchauyo_pb_service'
import * as actions from './fragments'
import { Code } from 'grpc-web-client/dist/Code'

describe('actions/fragments', () => {
  describe('queryTask', () => {
    const action = actions.fragments.query.started(new FragmentsQuery())

    it('works', async () => {
      const client = jest.fn()
      const response = new FragmentsQuery()
      response.setPrefix('meow')
      client.mockReturnValueOnce(Promise.resolve(new Ok(response)))

      const result = await actions.queryTaskInner(client)(action.payload)(jest.fn(), jest.fn())

      expect(client).toHaveBeenCalledWith(Yacchauyo.QueryFragments, action.payload)
      expect(result).toEqual(new Ok(response.toObject()))
    })

    it('can also fail', async () => {
      const client = jest.fn()
      const response = { status: Code.NotFound }
      client.mockReturnValueOnce(Promise.resolve(new Err(response)))

      const result = await actions.queryTaskInner(client)(action.payload)(jest.fn(), jest.fn())

      expect(client).toHaveBeenCalledWith(Yacchauyo.QueryFragments, action.payload)
      expect(result).toEqual(new Err(response))
    })
  })
})
