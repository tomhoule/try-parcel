import * as redux from 'redux'
import ReduxThunk from 'redux-thunk'
// import createSagaMiddleware from 'redux-saga'
import { texts } from './reducers/texts'
// import { rootSaga } from './sagas/root'

const composeEnhancers = (window as any).__REDUX_DEVTOOLS_EXTENSION_COMPOSE__ || redux.compose
// const saga = createSagaMiddleware()
const reducer = redux.combineReducers({
  texts,
})

export const store = redux.createStore(
  reducer,
  composeEnhancers(
    // redux.applyMiddleware(saga),
    redux.applyMiddleware(ReduxThunk),
  ),
)

// saga.run(rootSaga)
