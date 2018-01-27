import { SagaIterator } from 'redux-saga'
import { call, takeLatest } from 'redux-saga/effects'
import { texts } from '../actions/texts'
import { Action } from 'typescript-fsa'
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

export function* textsIndex(action: Action<{}>): SagaIterator {
  const payload = new proto.TextsQuery()
  const wrongPaylaoad = new proto.Text()
  const texts = rpcCall(
    backend.Yacchauyo.TextsIndex,
    payload
  ).then(
    texts => console.log(texts) 
  )
}

export function* rootSaga() {
  yield call(console.log, 'sagas are running')
  yield takeLatest(texts.fetchIndex.started.type, textsIndex)
}
