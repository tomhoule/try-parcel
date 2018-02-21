import { Code } from 'grpc-web-client/dist/Code'
import { Ok, Err } from '../prelude'
import { Yacchauyo } from '../rpc/yacchauyo_pb_service'
import * as actions from './schemas'
import { TextsQuery, Schema } from '../rpc/yacchauyo_pb'

describe('actions/fragments', () => {
  describe('textSchema', () => {
    const action = actions.schemas.textSchema.started(new TextsQuery())

    it('works', async () => {
      const response = new Schema()
      const client = jest.fn()
      client.mockReturnValueOnce(Promise.resolve(new Ok(response)))

      const result = await actions.textSchemaInner(client)(action.payload)(jest.fn(), jest.fn())

      expect(client).toHaveBeenCalledWith(Yacchauyo.TextSchema, action.payload)
      expect(result).toEqual(new Ok(response.toObject()))
    })
  })
})
