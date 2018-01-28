import { call } from 'redux-saga/effects'
import * as sagas from './root'
import { texts } from '../actions/texts'
import * as backend from '../rpc/yacchauyo_pb_service'
import * as proto from '../rpc/yacchauyo_pb'

describe('rootSaga', () => {
  const saga = sagas.rootSaga()

  it('forks textsIndex', () => {
    const next = saga.next()
    expect(next.value.FORK).toBeDefined()
  })

  it('forks createText', () => {
    const next = saga.next()
    expect(next.value.FORK).toBeDefined()
  })

  it('forks patchText', () => {
    const next = saga.next()
    expect(next.value.FORK).toBeDefined()
  })

  it('is then done', () => {
    const next = saga.next()
    expect(next.value).not.toBeDefined()
    expect(next.done).toBe(true)
  })
})

describe('textsIndex', () => {
  const action = texts.fetchIndex.started({})
  const saga = sagas.textsIndex(action)

  it('calls the api', () => {
    const next = saga.next()
    expect(next.value)
      .toEqual(
        call(sagas.rpcCall, backend.Yacchauyo.TextsIndex, new proto.TextsQuery()  )
      )
  })

  it('then returns the result of the call', () => {
    const result = new proto.Text()
    const next = saga.next({ message: result })
    expect(next.value).toEqual(result.toObject())
    expect(next.done).toBe(true)
  })
})

describe('createText', () => {
  const action = texts.create.started(new proto.Text())
  const saga = sagas.createText(action)

  it('calls the api', () => {
    const next = saga.next()
    expect(next.value)
      .toEqual(call(sagas.rpcCall, backend.Yacchauyo.CreateText, action.payload))
  })

  it('then returns the result of the call', () => {
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

  it('calls the api', () => {
    const next = saga.next()
    expect(next.value)
      .toEqual(call(sagas.rpcCall, backend.Yacchauyo.PatchText, action.payload))
  })

  it('then returns the result of the call', () => {
    const result = new proto.Text()
    result.setId('3333')
    const next = saga.next({ message: result })
    expect(next.value).toEqual(result.toObject())
    expect(next.done).toBe(true)
  })
})
