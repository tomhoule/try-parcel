import * as React from 'react'
import { shallow } from 'enzyme'
import { CreateText } from './CreateText'

describe('components/<CreateText />', () => {
  const props = {
    createText: jest.fn(),
    params: {},
    patch: jest.fn(),
  }
  const wrapper = shallow(<CreateText {...props} />)
  const wi: any = wrapper.instance()

  test('it renders', () => {
    expect(wrapper.length).toBe(1)
  })

  describe('submit', () => {
    describe('when creating a text', () => {
      test('it calls createText', () => {
        wi.submit()
        expect(wi.props.createText).toHaveBeenCalled()
      })
    })

    describe('when editing a text', () => {
      test('it calls patch', () => {
        wrapper.setProps({
          params: { textId: 'abc' },
        })
        wi.submit()
        const expected = new Text()
        expect(wi.props.patch).toHaveBeenCalled()
      })
    })
  })
})
