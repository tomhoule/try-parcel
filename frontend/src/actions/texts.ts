import { actionCreatorFactory } from 'typescript-fsa'
import * as proto from '../rpc/yacchauyo_pb'
import { buckle, rpcCall, Err, Ok, bindThunk } from '../prelude'
import * as backend from '../rpc/yacchauyo_pb_service'
import { Code } from 'grpc-web-client/dist/Code'

const factory = actionCreatorFactory('texts')

export const texts = {
  create: factory.async<proto.Text, proto.Text.AsObject, RpcFailure>('CREATE'),
  fetchIndex: factory.async<proto.TextsQuery, proto.Texts.AsObject, RpcFailure>('FETCH_INDEX'),
  fetchSingle: factory.async<string, TextSingle, RpcFailure>('FETCH_SINGLE'),
  patch: factory.async<proto.Text, proto.Text.AsObject, RpcFailure>('PATCH'),
  receive: factory<proto.Text>('RECEIVE'),
}

export const fetchIndexTask = buckle(
  texts.fetchIndex,
  async (action) => {
    const result = await rpcCall(backend.Yacchauyo.TextsIndex, action.payload)
    return result.map(res => res.toObject())
  })

export const createTaskInner = (client: typeof rpcCall) => buckle(
  texts.create,
  async (action, put: any, gs: any) => {
    const result = await client(backend.Yacchauyo.CreateText, action.payload)
    return result.map(res => res.toObject())
  })

export const createTask = bindThunk(createTaskInner(rpcCall))

const patchTaskInner = buckle(
  texts.patch,
  async (action) => {
    const result = await rpcCall(backend.Yacchauyo.PatchText, action.payload)
    return result.map(res => res.toObject())
  },
)

export const patchTask = bindThunk(patchTaskInner)

const fetchTaskInner = buckle(
  texts.fetchSingle,
  async (action, put) => {
    const query = new proto.TextsQuery()
    query.setId(action.payload)
    const results = await Promise.all([
      rpcCall(backend.Yacchauyo.TextsIndex, query),
      rpcCall(backend.Yacchauyo.TextSchema, query),
    ])

    const text = results[0]
      .map(texts => texts.toObject().textsList[0])
      .andThen<proto.Text.AsObject>(text => text
        ? new Ok(text)
        : new Err({
            status: Code.NotFound,
            statusMessage:  'not found',
          }),
      ).mapErr(err => {
        if (err.status === Code.NotFound) {
          // put(push('/'))
        }
        return err
      })

    return text.andThen(text => results[1].map(schema => ({
      schema: schema.toObject(),
      text,
    })))
  },
)

export const fetchTask = bindThunk(fetchTaskInner)
