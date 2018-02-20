import * as React from 'react'
import { shallow } from 'enzyme'
import { CreateText } from './CreateText'
import * as proto from '../rpc/yacchauyo_pb'

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
          patch: jest.fn(),
          text: { id: 'abcd' },
        })
        wi.submit()
        const expected = new proto.Text()
        expected.setAuthors('')
        expected.setDescription('')
        expected.setSlug('')
        expected.setTitle('')
        expected.setId('abcd')
        expect(wi.props.patch).toHaveBeenCalledWith(expected)
      })
    })
  })
})
