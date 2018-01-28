/// <reference path='../node_modules/@types/jest/index.d.ts' />
import { configure } from 'enzyme'
import Adapter = require('enzyme-adapter-react-16')

configure({ adapter: new Adapter() })
