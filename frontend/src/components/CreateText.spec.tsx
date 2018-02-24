import * as React from 'react'
import { shallow } from 'enzyme'
import { CreateText } from './CreateText'
import * as proto from '../rpc/yacchauyo_pb'
import { Ok } from '../prelude'

describe('components/<CreateText />', () => {
  const props = {
    create: jest.fn(),
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
    test('it calls create', () => {
      wrapper.setProps({ create: jest.fn() })
      const text = new proto.Text()
      text.setTitle('meow')
      wi.submit(text)
      expect(wi.props.create).toHaveBeenCalledWith(text)
    })
  })
})
