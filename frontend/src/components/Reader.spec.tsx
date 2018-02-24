import * as React from 'react'
import { shallow } from 'enzyme'
import { Reader } from './Reader'

describe('components/<Reader />', () => {
  const props = {
    fetchText: jest.fn(),
    match: {
      params: {
        textId: 'zzz',
      },
    },
    text: null,
  } as any
  const wrapper = shallow(<Reader {...props} />)
  const wi: any = wrapper.instance()

  it('renders', () => {
    expect(wrapper.length).toBe(1)
  })

  describe('on mount', () => {
    test('it fetches the text', () => {
      wrapper.setProps({
        fetchText: jest.fn(),
      })
      wi.componentWillMount()
      expect(wi.props.fetchText).toHaveBeenCalledWith('zzz')
    })
  })
})
