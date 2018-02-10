import * as React from 'react'
import * as enzyme from 'enzyme'
import SchemaEditor from './SchemaEditor'

describe('<SchemaEditor />', () => {
  const map = {}
  const wrapper = enzyme.shallow(<SchemaEditor data={map} />)

  test('it renders without crashing', () => {
    expect(wrapper.length).toBe(1)
  })
})
