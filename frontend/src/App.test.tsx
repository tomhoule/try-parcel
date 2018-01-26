import Adapter from 'enzyme-adapter-react-16'
import * as React from 'react'
import { App } from './App'
import { shallow } from 'enzyme'

test('it renders', () => {
  const wrapper = shallow(<App />)
  expect(wrapper.length).toBe(1)
})

test('adding works', () => {
  expect(3 + 5).toBe(8)
})
