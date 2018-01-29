import { call } from 'redux-saga/effects'
import { schemas } from '../actions/schemas'
import { texts } from '../actions/texts'
import * as proto from '../rpc/yacchauyo_pb'
import * as backend from '../rpc/yacchauyo_pb_service'
import * as sagas from './root'

describe('rootSaga', () => {
  const saga = sagas.rootSaga()

  test('forks textsIndex', () => {
    const next = saga.next()
    expect(next.value.FORK).toBeDefined()
  })

  test('forks createText', () => {
    const next = saga.next()
    expect(next.value.FORK).toBeDefined()
  })

  test('forks patchText', () => {
    const next = saga.next()
    expect(next.value.FORK).toBeDefined()
  })

  test('forks textSchema', () => {
    const next = saga.next()
    expect(next.value.FORK).toBeDefined()
  })

  test('forks patchSchema', () => {
    const next = saga.next()
    expect(next.value.FORK).toBeDefined()
  })

  test('is then done', () => {
    const next = saga.next()
    expect(next.value).not.toBeDefined()
    expect(next.done).toBe(true)
  })
})

describe('textsIndex', () => {
  const req = new proto.TextsQuery()
  const action = texts.fetchIndex.started(req)
  const saga = sagas.textsIndex(action)

  test('calls the api', () => {
    const next = saga.next()
    expect(next.value)
      .toEqual(
        call(sagas.rpcCall, backend.Yacchauyo.TextsIndex, new proto.TextsQuery()),
      )
  })

  test('then returns the result of the call', () => {
    const result = new proto.Text()
    const next = saga.next({ message: result })
    expect(next.value).toEqual(result.toObject())
    expect(next.done).toBe(true)
  })
})

describe('createText', () => {
  const action = texts.create.started(new proto.Text())
  const saga = sagas.createText(action)

  test('calls the api', () => {
    const next = saga.next()
    expect(next.value)
      .toEqual(call(sagas.rpcCall, backend.Yacchauyo.CreateText, action.payload))
  })

  test('then returns the result of the call', () => {
    const result = new proto.Text()
    result.setId('3333')
    const next = saga.next({ message: result })
    expect(next.value).toEqual(result.toObject())
    expect(next.done).toBe(true)
  })
})

describe('patchText', () => {
  const action = texts.patch.started(new proto.Text())
  const saga = sagas.patchText(action)

  test('calls the api', () => {
    const next = saga.next()
    expect(next.value)
      .toEqual(call(sagas.rpcCall, backend.Yacchauyo.PatchText, action.payload))
  })

  test('then returns the result of the call', () => {
    const result = new proto.Text()
    result.setId('3333')
    const next = saga.next({ message: result })
    expect(next.value).toEqual(result.toObject())
    expect(next.done).toBe(true)
  })
})

describe('textSchema', () => {
  const action = schemas.textSchema.started(new proto.TextsQuery())
  const saga = sagas.textSchema(action)

  test('calls the api', () => {
    const next = saga.next()
    expect(next.value)
      .toEqual(call(sagas.rpcCall, backend.Yacchauyo.TextSchema, action.payload))
  })

  test('then returns the result of the call', () => {
    const result = new proto.Schema()
    result.setId('3333')
    const next = saga.next({ message: result })
    expect(next.value).toEqual(result.toObject())
    expect(next.done).toBe(true)
  })
})

describe('patchSchema', () => {
  const action = schemas.patchSchema.started(new proto.Schema())
  const saga = sagas.patchSchema(action)

  test('calls the api', () => {
    const next = saga.next()
    expect(next.value)
      .toEqual(call(sagas.rpcCall, backend.Yacchauyo.PatchSchema, action.payload))
  })

  test('then returns the result of the call', () => {
    const result = new proto.Schema()
    result.setId('3333')
    const next = saga.next({ message: result })
    expect(next.value).toEqual(result.toObject())
    expect(next.done).toBe(true)
  })
})
