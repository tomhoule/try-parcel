import { SagaIterator } from 'redux-saga'
import { call, put, takeLatest } from 'redux-saga/effects'
import { texts } from '../actions/texts'
import { schemas } from '../actions/schemas'
import { Action, AsyncActionCreators } from 'typescript-fsa'
import * as proto from '../rpc/yacchauyo_pb'
import * as backend from '../rpc/yacchauyo_pb_service'
import { grpc, Code } from 'grpc-web-client'
import { API_URL } from '../config'
import { Message } from 'google-protobuf';

export function rpcCall<
  Request extends Message,
  Response,
  Method extends grpc.UnaryMethodDefinition<Request, Message>
  >(
  method: Method,
  request: Request
  ): Promise<grpc.UnaryOutput<Message>> {
  return new Promise((resolve, reject) => grpc.unary(method, {
    request,
    host: API_URL,
    onEnd: (message) => {
      if (message.status !== Code.OK) {
        console.log('Error response: ', message)
        reject(message)
      } else {
        resolve(message)
      }
    }
  }))
}

export function buckle<
  S,
  D,
  F
>(
  actionCreators: AsyncActionCreators<S, D, F>,
  inner: (action: Action<S>) => SagaIterator,
): (action: Action<S>) => SagaIterator {
  return function* buckled(action: Action<S>) {
    try {
      const result = yield* inner(action)
      yield put(actionCreators.done({
        params: action.payload,
        result,
      }))
    } catch (err) {
      yield put(actionCreators.failed({
        params: action.payload,
        error: err,
      }))
    }
  }
}

export function* textsIndex(action: Action<{}>): SagaIterator {
  const payload = new proto.TextsQuery()
  const response = yield call(rpcCall, backend.Yacchauyo.TextsIndex, payload)
  return response.message.toObject()
}

export function* createText(action: Action<proto.Text>): SagaIterator {
  const response = yield call(rpcCall, backend.Yacchauyo.CreateText, action.payload)
  return response.message.toObject()
}

export function* patchText(action: Action<proto.Text>): SagaIterator {
  const response = yield call(rpcCall, backend.Yacchauyo.PatchText, action.payload)
  return response.message.toObject()
}

export function* textSchema(action: Action<proto.TextsQuery>): SagaIterator {
  const response = yield call(rpcCall, backend.Yacchauyo.TextSchema, action.payload)
  return response.message.toObject()
}

export function* rootSaga() {
  yield takeLatest(texts.fetchIndex.started.type, buckle(texts.fetchIndex, textsIndex))
  yield takeLatest(texts.create.started.type, buckle(texts.create, createText))
  yield takeLatest(texts.patch.started.type, buckle(texts.patch, patchText))
  yield takeLatest(schemas.textSchema.type, buckle(schemas.textSchema, textSchema))
}
