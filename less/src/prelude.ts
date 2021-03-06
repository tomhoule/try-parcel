import { Message } from 'google-protobuf'
import { grpc } from 'grpc-web-client'
import { Code } from 'grpc-web-client/dist/Code'
import { API_URL } from './config'

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
  private readonly value: T

  constructor(value: T) { this.value = value }

  isOk() { return true }
  isErr() { return false }
  ok() { return this.value }
  err() { return null }
  map<U>(op: (value: T) => U): Result<U, Error> { return new Ok(op(this.value)) }
  mapErr<U>(fn: (err: Error) => U): Result<T, U> { return new Ok(this.value) }
  and<U>(other: Result<U, Error>): Result<U, Error> { return other }
  andThen<U>(op: (success: T) => Result<U, Error>): Result<U, Error> { return op(this.value) }
  or<U>(other: Result<T, U>): Result<T, U> { return new Ok(this.value) }
  orElse<U>(op: (inner: Error) => Result<T, U>): Result<T, U> { return new Ok(this.value) }
  unwrapOr<U>(fallback: T): T { return this.value }
}

// tslint:disable-next-line:max-classes-per-file
export class Err<Succ, T> implements Result<Succ, T> {
  private readonly value: T

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

export function rpcCall<
  Response extends Message,
  Request extends Message
  >(
    method: grpc.UnaryMethodDefinition<Request, Response>,
    request: Request,
): Promise<Result<Response, RpcFailure>> {
  return new Promise((resolve) => grpc.unary(method, {
    host: API_URL,
    onEnd: (message: grpc.UnaryOutput<Response>) => {
      if (message.status !== Code.OK || !message.message) {
        resolve(new Err({ status: message.status, statusMessage: message.statusMessage }))
      } else {
        resolve(new Ok(message.message))
      }
    },
    request,
  }))
}
