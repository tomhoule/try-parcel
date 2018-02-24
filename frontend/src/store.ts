import * as redux from 'redux'
import ReduxThunk from 'redux-thunk'
import { texts } from './reducers/texts'
import { fragments } from './reducers/fragments'

const composeEnhancers = (window as any).__REDUX_DEVTOOLS_EXTENSION_COMPOSE__ || redux.compose
const reducer = redux.combineReducers({
  fragments,
  texts,
})

export const store = redux.createStore(
  reducer,
  composeEnhancers(
    redux.applyMiddleware(ReduxThunk),
  ),
)
