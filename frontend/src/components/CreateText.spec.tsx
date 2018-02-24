import * as React from 'react'
import { shallow } from 'enzyme'
import { CreateText } from './CreateText'
import * as proto from '../rpc/yacchauyo_pb'
import { Ok } from '../prelude'

describe('components/<CreateText />', () => {
  const props = {
    history: {} as any,
    location: {} as any,
    match: {} as any,
  }
  const wrapper = shallow(<CreateText {...props} />)
  const wi: any = wrapper.instance()

  test('it renders', () => {
    expect(wrapper.length).toBe(1)
  })

  describe('submit', () => {
    test('it calls CreateText', () => {
      expect(true).toBe(false)
    })
  })
})
