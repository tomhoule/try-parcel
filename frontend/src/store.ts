import * as redux from 'redux'
import ReduxThunk from 'redux-thunk'
import { texts } from './reducers/texts'

const composeEnhancers = (window as any).__REDUX_DEVTOOLS_EXTENSION_COMPOSE__ || redux.compose
const reducer = redux.combineReducers({
  texts,
})

export const store = redux.createStore(
  reducer,
  composeEnhancers(
    redux.applyMiddleware(ReduxThunk),
  ),
)
