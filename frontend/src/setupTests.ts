import { configure } from 'enzyme'
import Adapter = require('enzyme-adapter-react-16')
import 'jest'

configure({ adapter: new Adapter() })
