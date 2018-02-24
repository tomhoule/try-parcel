import * as React from 'react'
import { shallow } from 'enzyme'
import { TextForm } from './TextForm'
import * as proto from '../rpc/yacchauyo_pb'
import { Ok } from '../prelude'

describe('components/<TextForm />', () => {
  const props = {
    submit: jest.fn(),
  }
  const wrapper = shallow(<TextForm {...props} />)
  const wi: any = wrapper.instance()

  test('it renders', () => {
    expect(wrapper.length).toBe(1)
  })

  describe('submit', () => {
    describe('on success', () => {
      test('calls submit from the props', () => {
        const response = new Text()
        const rpcResult = Promise.resolve(new Ok(response))
        wrapper.setProps({
          submit: jest.fn().mockReturnValueOnce(rpcResult),
        })
        wi.submit()
        expect(wi.props.submit).toHaveBeenCalled()
      })
    })

    // describe('when editing a text', () => {
    //   test('it calls patch', () => {
    //     wrapper.setProps({
    //       patch: jest.fn(),
    //       text: { id: 'abcd' },
    //     })
    //     wi.submit()
    //     const expected = new proto.Text()
    //     expected.setAuthors('')
    //     expected.setDescription('')
    //     expected.setSlug('')
    //     expected.setTitle('')
    //     expected.setId('abcd')
    //     expect(wi.props.patch).toHaveBeenCalledWith(expected)
    //   })
    // })
  })
})
