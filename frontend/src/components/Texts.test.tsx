import { shallow } from 'enzyme'
import 'jest'
import * as React from 'react'
import { TextsQuery } from '../rpc/yacchauyo_pb'
import { Texts } from './Texts'

describe('components/<CreateText />', () => {
  const props = {
    fetchIndex: jest.fn() as any,
    texts: [],
  }
  const wrapper = shallow(<Texts {...props} />)
  const wi: any = wrapper.instance()

  beforeEach(() => {
    wi.props.fetchIndex.mockReset()
  })

  test('renders', () => {
    expect(wrapper.length).toBe(1)
  })

  describe('on mount', () => {
    test('it fetches the texts index', () => {
      wi.componentWillMount()
      expect(wi.props.fetchIndex).toHaveBeenCalledWith(new TextsQuery())
    })
  })
})
