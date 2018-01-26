import * as redux from 'redux'
import createSagaMiddleware from 'redux-saga'
import { rootSaga } from './sagas/root'
import { texts } from './reducers/texts'

const composeEnhancers = (window as any).__REDUX_DEVTOOLS_EXTENSION_COMPOSE__ || redux.compose
const saga = createSagaMiddleware()
const reducer = redux.combineReducers({
  texts,
})

export const store = redux.createStore(
  reducer,
  composeEnhancers(redux.applyMiddleware(saga))
)

saga.run(rootSaga)
