import * as React from 'react'
import { App } from './App'
import { shallow } from 'enzyme'

describe('<App />', () => {
  it('renders', () => {
    const wrapper = shallow(<App />)
    expect(wrapper.length).toBe(1)
  })
})
