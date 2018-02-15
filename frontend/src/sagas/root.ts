import { Message } from 'google-protobuf'
import { grpc } from 'grpc-web-client'
import { Code } from 'grpc-web-client/dist/Code'
import { SagaIterator } from 'redux-saga'
import { call, put, takeLatest } from 'redux-saga/effects'
import { Action, AsyncActionCreators } from 'typescript-fsa'
import { schemas } from '../actions/schemas'
import { texts } from '../actions/texts'
import { API_URL } from '../config'
import * as proto from '../rpc/yacchauyo_pb'
import * as backend from '../rpc/yacchauyo_pb_service'

/**
 * https://doc.rust-lang.org/beta/std/result/enum.Result.html
 */
export interface Result<Success, Failure> {
  isOk: () => boolean
  isErr: () => boolean
  ok: () => Success | null
  err: () => Failure | null
  map: <T>(fn: (success: Success) => T) => Result<T, Failure>
  mapErr: <T>(fn: (failure: Failure) => T) => Result<Success, T>
  and: <T>(res: Result<T, Failure>) => Result<T, Failure>
  andThen: <T>(op: (inner: Success) => Result<T, Failure>) => Result<T, Failure>
  or: <T>(res: Result<Success, T>) => Result<Success, T>
  orElse: <T>(op: (inner: Failure) => Result<Success, T>) => Result<Success, T>
  unwrapOr: <T>(fallback: Success) => Success
}

export class Ok<T, Error> implements Result<T, Error> {
  value: T

  constructor(value: T) { this.value = value }

  isOk() { return true }
  isErr() { return false }
  ok() { return this.value }
  err() { return null }
  map<U>(op: (value: T) => U): Result<U, Error> { return new Ok(op(this.value))}
  mapErr<U>(fn: (err: Error) => U): Result<T, U> { return new Ok(this.value) }
  and<U>(other: Result<U, Error>): Result<U, Error> { return other }
  andThen<U>(op: (success: T) => Result<U, Error>): Result<U, Error> { return op(this.value) }
  or<U>(other: Result<T, U>): Result<T, U> { return new Ok(this.value) }
  orElse<U>(op: (inner: Error) => Result<T, U>): Result<T, U> { return new Ok(this.value) }
  unwrapOr<U>(fallback: T): T { return this.value }
}

// tslint:disable-next-line:max-classes-per-file
export class Err<Succ, T> implements Result<Succ, T> {
  value: T

  constructor(value: T) { this.value = value }
  isOk() { return false }
  isErr() { return true }
  ok() { return null }
  err() { return this.value }
  map<U>(op: (value: Succ) => U): Result<U, T> { return new Err(this.value) }
  mapErr<U>(op: (error: T) => U): Result<Succ, U> { return new Err(op(this.value)) }
  and<U>(other: Result<U, T>): Result<U, T> { return new Err(this.value) }
  andThen<U>(op: (succ: Succ) => Result<U, T>): Result<U, T> { return new Err(this.value) }
  or<U>(other: Result<Succ, U>): Result<Succ, U> { return other }
  orElse<U>(op: (error: T) => Result<Succ, U>): Result<Succ, U> { return op(this.value) }
  unwrapOr(fallback: Succ): Succ { return fallback }
}

interface RpcFailure {
  status: Code
  statusMessage: string
}

export function rpcCall<
  Response extends Message,
  Request extends Message
  >(
  method: grpc.UnaryMethodDefinition<Request, Response>,
  request: Request,
): Promise<Result<Response, RpcFailure>> {
  return new Promise((resolve) => grpc.unary(method, {
    host: API_URL,
    onEnd: (message: grpc.UnaryOutput<Response  >) => {
      if (message.status !== Code.OK || !message.message) {
        resolve(new Err({ status: message.status, statusMessage: message.statusMessage }))
      } else {
        resolve(new Ok(message.message))
      }
    },
    request,
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
        error: err,
        params: action.payload,
      }))
    }
  }
}

export function* textsIndex(action: Action<{}>): any {
  const payload = new proto.TextsQuery()
  // const response = await rpcCall(backend.Yacchauyo.TextsIndex, payload)
  // return response.map(msg => msg.toObject())
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

export function* patchSchema(action: Action<proto.Schema>): SagaIterator {
  const response = yield call(rpcCall, backend.Yacchauyo.PatchSchema, action.payload)
  return response.message.toObject()
}

export function* rootSaga() {
  yield takeLatest(texts.fetchIndex.started.type, buckle(texts.fetchIndex, textsIndex))
  yield takeLatest(texts.create.started.type, buckle(texts.create, createText))
  yield takeLatest(texts.patch.started.type, buckle(texts.patch, patchText))
  yield takeLatest(schemas.textSchema.type, buckle(schemas.textSchema, textSchema))
  yield takeLatest(schemas.patchSchema.type, buckle(schemas.patchSchema, patchSchema))
}
