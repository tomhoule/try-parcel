import * as React from 'react'
import * as enzyme from 'enzyme'
import { Schema } from '../rpc/yacchauyo_pb'
import SchemaEditor from './SchemaEditor'

describe('<SchemaEditor />', () => {
  const schema = new Schema()
  schema.setId('abcd')
  schema.setPathsList(['ah', 'beh'])
  const props = {
    patchSchema: jest.fn(),
    schema: schema.toObject(),
  }
  const wrapper = enzyme.shallow(<SchemaEditor {...props} />)
  const wi: any = wrapper.instance()

  test('it renders without crashing', () => {
    expect(wrapper.length).toBe(1)
  })

  describe('on submit', () => {
    test('it patches with the pathList and the id', () => {
      const patchSchema = jest.fn()
      wrapper.setProps({ patchSchema })
      wrapper.setState({ paths: ['meow', 'index'] })
      wi.submit()
      const expected = new Schema()
      expected.setId(props.schema.id)
      expected.setPathsList(['meow', 'index'])
      expect(patchSchema).toHaveBeenCalledWith(expected)
    })
  })
})
