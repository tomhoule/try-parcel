import { SagaIterator } from 'redux-saga'
import { call, takeLatest } from 'redux-saga/effects'
import { texts } from '../actions/texts'
import { Action } from 'typescript-fsa'
import * as proto from '../rpc/yacchauyo_pb'
import * as backend from '../rpc/yacchauyo_pb_service'
import { grpc, Code } from 'grpc-web-client'
import { API_URL } from '../config'
import { Message } from 'google-protobuf';

export function callBackend<
  Response extends Method['responseType'],
  Request extends Message,
  Method extends grpc.UnaryMethodDefinition<Request, Message & Response>
>(
  method: Method,
  request: Request
): Promise<Response> {
  return new Promise((resolve, reject) => grpc.unary(method, {
    request,
    host: API_URL,
    metadata: {
    },
    onEnd: (message: grpc.UnaryOutput<any>) => {
      if (message.status !== Code.OK) {
        console.log('Error response: ', message)
        reject(message)
      } else {
        if (message.message) {
          resolve(message.message as Response)
        } else {
          reject(message)
        }
      }
    }
  }))
}

export function* textsIndex(action: Action<{}>): SagaIterator {
  const payload = new proto.TextsQuery()
  const wrongPaylaoad = new proto.Text()
  const texts = callBackend(
    backend.Yacchauyo.TextsIndex,
    payload
  ).then(
    (texts: typeof proto.Texts) => {
      const obj = texts.toObject(true, undefined as any)
      console.log(obj)
    }
  )
}

export function* rootSaga() {
  yield call(console.log, 'sagas are running')
  yield takeLatest(texts.fetchIndex.started.type, textsIndex)
}
